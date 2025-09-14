use crate::{
    ast::expr::{Expr, ExprKind},
    cfg::{Instruction, ValueId},
    hir_builder::{
        errors::{SemanticError, SemanticErrorKind},
        types::checked_type::{Type, TypeKind},
        utils::scope::SymbolEntry,
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_lvalue_expr(&mut self, ctx: &mut HIRContext, expr: Expr) -> Result<ValueId, SemanticError> {
        match expr.kind {
            ExprKind::Identifier(identifier) => {
                if let Some(SymbolEntry::VarDecl(decl)) = ctx.module_builder.scope_lookup(identifier.name) {
                    return Ok(decl.value_id); // ValueId which holds Pointer<T>
                } else {
                    return Err(SemanticError {
                        kind: SemanticErrorKind::UndeclaredIdentifier(identifier),
                        span: expr.span,
                    });
                }
            }
            ExprKind::Access { left, field } => {
                let base_ptr_id = self.build_lvalue_expr(ctx, *left)?;
                let base_ptr_type = self.get_value_id_type(&base_ptr_id);

                if let TypeKind::Pointer(ptr_to) = &base_ptr_type.kind {
                    if let TypeKind::Struct(struct_decl) = &ptr_to.kind {
                        if let Some((field_index, field)) = struct_decl
                            .fields
                            .iter()
                            .enumerate()
                            .find(|(_, f)| f.identifier.name == field.name)
                        {
                            let field_ptr_id = self.new_value_id();
                            self.cfg.value_types.insert(
                                field_ptr_id,
                                Type {
                                    kind: TypeKind::Pointer(Box::new(field.constraint.clone())),
                                    span: expr.span,
                                },
                            );

                            self.add_basic_block_instruction(Instruction::GetFieldPtr {
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
                            kind: SemanticErrorKind::CannotAccess(ptr_to.as_ref().clone()),
                            span: expr.span,
                        });
                    }
                } else {
                    panic!("INTERNAL COMPILER ERROR: Expected base_ptr_id to be of Pointer<T> type");
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

    pub fn build_assignment_stmt(&mut self, ctx: &mut HIRContext, target: Expr, value: Expr) {
        let source_val = self.build_expr(ctx, value);

        let destination_ptr = match self.build_lvalue_expr(ctx, target) {
            Ok(value_id) => value_id,
            Err(e) => {
                ctx.module_builder.errors.push(e);
                return;
            }
        };

        self.emit_store(ctx, destination_ptr, source_val);
    }
}
