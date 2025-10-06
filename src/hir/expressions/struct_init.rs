use std::collections::HashSet;

use crate::{
    ast::{expr::Expr, IdentifierNode, Span},
    hir::{
        cfg::Value,
        errors::{SemanticError, SemanticErrorKind},
        types::checked_type::TypeKind,
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_struct_init_expr(&mut self, ctx: &mut HIRContext, fields: Vec<(IdentifierNode, Expr)>, span: Span) -> Value {
        let mut initialized_fields: HashSet<IdentifierNode> = HashSet::new();

        let struct_ptr = self.emit_stack_alloc(ctx, left_type.clone(), 1);

        for (field_name, field_expr) in fields {
            if !initialized_fields.insert(field_name) {
                return Value::Use(self.report_error_and_get_poison(
                    ctx,
                    SemanticError {
                        kind: SemanticErrorKind::DuplicateStructFieldInitializer(field_name),
                        span: field_name.span,
                    },
                ));
            }

            let field_expr_span = field_expr.span;

            let field_ptr = match self.emit_get_field_ptr(ctx, struct_ptr, field_name) {
                Ok(ptr) => ptr,
                Err(error) => return Value::Use(self.report_error_and_get_poison(ctx, error)),
            };
            let field_ptr_type = ctx.program_builder.get_value_id_type(&field_ptr);

            let field_value = self.build_expr(ctx, field_expr);
            let field_value_type = ctx.program_builder.get_value_type(&field_value);

            if let TypeKind::Pointer(expected_field_type) = field_ptr_type.kind {
                if !self.check_is_assignable(&field_value_type, &expected_field_type) {
                    return Value::Use(self.report_error_and_get_poison(
                        ctx,
                        SemanticError {
                            span: field_expr_span,
                            kind: SemanticErrorKind::TypeMismatch {
                                expected: *expected_field_type,
                                received: field_value_type,
                            },
                        },
                    ));
                }
            } else {
                panic!("INTERNAL COMPILER ERROR: struct field pointer must be a pointer");
            }

            self.emit_store(ctx, field_ptr, field_value);
        }

        let final_struct_value_id = self.emit_load(ctx, struct_ptr);
        return Value::Use(final_struct_value_id);
    }
}
