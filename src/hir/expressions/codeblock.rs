use crate::{
    ast::expr::BlockContents,
    hir::{cfg::Value, utils::scope::ScopeKind, FunctionBuilder, HIRContext},
};

impl FunctionBuilder {
    pub fn build_codeblock_expr(
        &mut self,
        ctx: &mut HIRContext,
        codeblock: BlockContents,
    ) -> Value {
        ctx.module_builder.enter_scope(ScopeKind::CodeBlock);

        self.build_statements(ctx, codeblock.statements);

        let result_value = if let Some(final_expr) = codeblock.final_expr {
            self.build_expr(ctx, *final_expr)
        } else {
            Value::VoidLiteral
        };

        ctx.module_builder.exit_scope();

        result_value
    }
}
