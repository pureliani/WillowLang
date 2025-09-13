use crate::{
    ast::{expr::Expr, IdentifierNode},
    cfg::{Instruction, Value},
    hir_builder::{
        errors::{SemanticError, SemanticErrorKind},
        types::checked_type::{Type, TypeKind},
        FunctionBuilder, ModuleBuilder,
    },
};

impl FunctionBuilder {
    pub fn build_access_expr(&mut self, module_builder: &mut ModuleBuilder, left: Box<Expr>, field: IdentifierNode) -> Value {
        let base_ptr_id = match self.build_lvalue_expr(module_builder, *left) {
            Ok(id) => id,
            Err(e) => return self.report_error_and_get_poison(module_builder, e),
        };

        let base_ptr_type = self.get_value_id_type(&base_ptr_id);

        if let TypeKind::Pointer(ptr_to) = &base_ptr_type.kind {
            if let TypeKind::Struct(struct_decl) = &ptr_to.kind {
                if let Some((field_index, checked_field)) = struct_decl
                    .fields
                    .iter()
                    .enumerate()
                    .find(|(_, f)| f.identifier.name == field.name)
                {
                    let field_ptr_id = self.new_value_id();
                    self.cfg.value_types.insert(
                        field_ptr_id,
                        Type {
                            kind: TypeKind::Pointer(Box::new(checked_field.constraint.clone())),
                            span: field.span,
                        },
                    );

                    self.add_basic_block_instruction(Instruction::FieldPtr {
                        destination: field_ptr_id,
                        base_ptr: base_ptr_id,
                        field_index,
                    });

                    let final_value_id = self.new_value_id();
                    self.cfg.value_types.insert(final_value_id, checked_field.constraint.clone());
                    self.add_basic_block_instruction(Instruction::Load {
                        destination: final_value_id,
                        source_ptr: field_ptr_id,
                    });

                    return Value::Use(final_value_id);
                } else {
                    return self.report_error_and_get_poison(
                        module_builder,
                        SemanticError {
                            kind: SemanticErrorKind::AccessToUndefinedField { field },
                            span: field.span,
                        },
                    );
                }
            }
        }

        self.report_error_and_get_poison(
            module_builder,
            SemanticError {
                kind: SemanticErrorKind::CannotAccess(base_ptr_type),
                span: field.span,
            },
        )
    }
}
