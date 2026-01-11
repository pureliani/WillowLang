use crate::{
    ast::{expr::Expr, Span},
    hir::{
        cfg::Terminator,
        errors::{SemanticError, SemanticErrorKind},
        types::checked_type::Type,
        utils::check_is_assignable::check_is_assignable,
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_return_stmt(&mut self, ctx: &mut HIRContext, value: Expr, span: Span) {
        let return_value = self.build_expr(ctx, value);
        let return_type = ctx.program_builder.get_value_type(&return_value);

        if !check_is_assignable(&return_type, &self.return_type) {
            ctx.module_builder.errors.push(SemanticError {
                kind: SemanticErrorKind::ReturnTypeMismatch {
                    expected: self.return_type.clone(),
                    received: return_type,
                },
                span,
            });
        }

        let final_value = if self.return_type == Type::Void {
            None
        } else {
            Some(return_value)
        };

        self.set_basic_block_terminator(Terminator::Return { value: final_value });
    }
}
