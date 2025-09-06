use crate::{
    ast::expr::{Expr, ExprKind},
    cfg::{Instruction, ValueId},
    hir_builder::{
        errors::{SemanticError, SemanticErrorKind},
        types::checked_type::{Type, TypeKind},
        utils::scope::SymbolEntry,
        HIRBuilder,
    },
};

impl<'a> HIRBuilder<'a> {
    pub fn build_lvalue_expr(&mut self, expr: Expr) -> Result<ValueId, SemanticError> {
        match expr.kind {
            ExprKind::Identifier { identifier } => {
                if let Some(SymbolEntry::VarDecl(decl)) = self.scope_lookup(identifier.name) {
                    return Ok(decl.value_id); // ValueId which holds Pointer<T>
                } else {
                    return Err(SemanticError {
                        kind: SemanticErrorKind::UndeclaredIdentifier(identifier),
                        span: expr.span,
                    });
                }
            }
            ExprKind::Access { left, field } => {
                let base_ptr_id = self.build_lvalue_expr(*left)?;
                let base_ptr_type = self.get_value_id_type(&base_ptr_id);

                if let TypeKind::Pointer(ptr_to) = &base_ptr_type.kind {
                    if let TypeKind::Struct(fields) = &ptr_to.kind {
                        if let Some((field_index, field)) =
                            fields.iter().enumerate().find(|(_, f)| f.identifier.name == field.name)
                        {
                            let field_ptr_id = self.new_value_id();
                            self.cfg.value_types.insert(
                                field_ptr_id,
                                Type {
                                    kind: TypeKind::Pointer(Box::new(field.constraint.clone())),
                                    span: expr.span,
                                },
                            );

                            self.add_basic_block_instruction(Instruction::FieldPtr {
                                destination: field_ptr_id,
                                base_ptr: base_ptr_id,
                                field_index,
                            });

                            return Ok(field_ptr_id);
                        } else {
                            return Err(SemanticError {
                                kind: SemanticErrorKind::AccessToUndefinedField { field },
                                span: expr.span,
                            });
                        }
                    } else {
                        return Err(SemanticError {
                            kind: SemanticErrorKind::CannotAccess(base_ptr_type),
                            span: expr.span,
                        });
                    }
                } else {
                    panic!("Expected base_ptr_id to be of Pointer<T> type");
                }
            }
            _ => {
                return Err(SemanticError {
                    kind: SemanticErrorKind::InvalidLValue,
                    span: expr.span,
                });
            }
        }
    }

    pub fn build_assignment_stmt(&mut self, target: Expr, value: Expr) {
        let source_val = self.build_expr(value);
        let source_type = self.get_value_type(&source_val);

        let destination_ptr_id = match self.build_lvalue_expr(target) {
            Ok(value_id) => value_id,
            Err(e) => {
                self.errors.push(e);
                return;
            }
        };
        let destination_ptr_type = self.get_value_id_type(&destination_ptr_id);

        if let TypeKind::Pointer(target_type) = destination_ptr_type.kind {
            if !self.check_is_assignable(&source_type, &target_type) {
                self.errors.push(SemanticError {
                    span: source_type.span,
                    kind: SemanticErrorKind::TypeMismatch {
                        expected: *target_type,
                        received: source_type,
                    },
                });
                return;
            }
        } else {
            panic!("Expected destination_ptr_id to be of Pointer<T> type");
        }

        self.add_basic_block_instruction(Instruction::Store {
            destination_ptr: destination_ptr_id,
            source_val,
        });
    }
}
