use crate::{
    ast::{
        base::base_expression::BlockContents,
        checked::{
            checked_expression::CheckedBlockContents,
            checked_type::{Type, TypeKind},
        },
    },
    check::{utils::scope::ScopeKind, SemanticChecker},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_codeblock(&mut self, block_contents: BlockContents) -> (Type, CheckedBlockContents) {
        self.enter_scope(ScopeKind::CodeBlock);
        let checked_codeblock_statements = self.check_stmts(block_contents.statements);
        let checked_codeblock_final_expr = block_contents.final_expr.map(|fe| {
            let checked_final_expr = self.check_expr(*fe);

            Box::new(checked_final_expr)
        });
        self.exit_scope();

        let ty = checked_codeblock_final_expr.clone().map(|fe| fe.ty).unwrap_or(Type {
            kind: TypeKind::Void,
            span: block_contents.span,
        });

        (
            ty,
            CheckedBlockContents {
                final_expr: checked_codeblock_final_expr,
                statements: checked_codeblock_statements,
            },
        )
    }
}
