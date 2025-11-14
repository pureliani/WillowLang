use crate::{
    ast::{expr::Expr, Span},
    hir::{
        cfg::Terminator,
        errors::{SemanticError, SemanticErrorKind},
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_return_stmt(&mut self, ctx: &mut HIRContext, value: Expr, span: Span) {
        let return_value = self.build_expr(ctx, value);
        let return_type = ctx.program_builder.get_value_type(&return_value);

        if !self.check_is_assignable(&return_type, &self.return_type) {
            ctx.module_builder.errors.push(SemanticError {
                kind: SemanticErrorKind::ReturnTypeMismatch {
                    expected: self.return_type.clone(),
                    received: return_type,
                },
                span,
            });
            return;
        }

        self.set_basic_block_terminator(Terminator::Return {
            value: Some(return_value),
        });

        let new_block = self.new_basic_block();
        self.use_basic_block(new_block);
    }
}
