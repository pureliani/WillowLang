use crate::{
    ast::{expr::BlockContents, stmt::StmtKind},
    hir::{
        cfg::Value, types::checked_declaration::CheckedDeclaration,
        utils::scope::ScopeKind, FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_codeblock_expr(
        &mut self,
        ctx: &mut HIRContext,
        codeblock: BlockContents,
    ) -> Value {
        ctx.module_builder.enter_scope(ScopeKind::CodeBlock);

        for stmt in &codeblock.statements {
            if let StmtKind::VarDecl(var_decl) = &stmt.kind {
                ctx.module_builder.scope_insert(
                    ctx.program_builder,
                    var_decl.identifier,
                    CheckedDeclaration::UninitializedVar {
                        id: ctx.program_builder.new_declaration_id(),
                        identifier: var_decl.identifier,
                    },
                );
            }
        }

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
