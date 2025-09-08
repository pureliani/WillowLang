use crate::{
    ast::expr::BlockContents,
    cfg::Value,
    hir_builder::{utils::scope::ScopeKind, FunctionBuilder},
};

impl<'a> FunctionBuilder<'a> {
    pub fn build_codeblock_expr(&mut self, codeblock: BlockContents) -> Value {
        self.enter_scope(ScopeKind::CodeBlock);
        self.build_statements(codeblock.statements);

        let result_value = if let Some(final_expr) = codeblock.final_expr {
            self.build_expr(*final_expr)
        } else {
            Value::VoidLiteral
        };

        self.exit_scope();

        result_value
    }
}
