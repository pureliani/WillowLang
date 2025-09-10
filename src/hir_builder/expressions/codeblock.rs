use crate::{
    ast::expr::BlockContents,
    cfg::Value,
    hir_builder::{utils::scope::ScopeKind, FunctionBuilder, ModuleBuilder},
};

impl FunctionBuilder {
    pub fn build_codeblock_expr(&mut self, module_builder: &mut ModuleBuilder, codeblock: BlockContents) -> Value {
        module_builder.enter_scope(ScopeKind::CodeBlock);
        self.build_statements(module_builder, codeblock.statements);

        let result_value = if let Some(final_expr) = codeblock.final_expr {
            self.build_expr(module_builder, *final_expr)
        } else {
            Value::VoidLiteral
        };

        module_builder.exit_scope();

        result_value
    }
}
