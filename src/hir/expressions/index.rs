// src/hir/expressions/index.rs

use crate::{
    ast::expr::Expr,
    hir::{
        cfg::Value,
        errors::{SemanticError, SemanticErrorKind},
        types::checked_type::{StructKind, Type},
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_index_expr(
        &mut self,
        ctx: &mut HIRContext,
        left: Box<Expr>,
        index: Box<Expr>,
    ) -> Value {
        let left_span = left.span;
        let index_span = index.span;

        let list_val = self.build_expr(ctx, *left);
        let list_type = ctx.program_builder.get_value_type(&list_val);

        let index_val = self.build_expr(ctx, *index);
        let index_type = ctx.program_builder.get_value_type(&index_val);

        // TODO: allow smaller unsigned types
        if !self.check_is_assignable(&index_type, &Type::USize) {
            return Value::Use(self.report_error_and_get_poison(
                ctx,
                SemanticError {
                    kind: SemanticErrorKind::TypeMismatch {
                        expected: Type::USize,
                        received: index_type,
                    },
                    span: index_span,
                },
            ));
        }

        todo!()
    }
}
