use crate::{
    ast::{
        base::base_expression::BlockContents,
        checked::{
            checked_expression::{CheckedBlockContents, CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
    },
    check::{utils::scope::ScopeKind, SemanticChecker},
    tfg::{TFGNodeId, TFGNodeKind},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_block_expr(
        &mut self,
        block_contents: BlockContents,
        span: crate::ast::Span,
        entry_node: TFGNodeId,
        next_node_if_true: TFGNodeId,
        next_node_if_false: TFGNodeId,
    ) -> CheckedExpr {
        self.enter_scope(ScopeKind::CodeBlock);

        let (block_type, checked_final_expr, checked_statements) = if let Some(final_expr) = block_contents.final_expr {
            let final_expr_entry_node = self.tfg().graph.create_node(TFGNodeKind::NoOp);

            let checked_stmts = self.check_stmts(block_contents.statements, entry_node, final_expr_entry_node);

            let checked_expr = self.check_expr(*final_expr, final_expr_entry_node, next_node_if_true, next_node_if_false);

            (checked_expr.ty.clone(), Some(Box::new(checked_expr)), checked_stmts)
        } else {
            let merge_node = self.tfg().graph.create_node(TFGNodeKind::NoOp);
            self.tfg().graph.link(merge_node, next_node_if_true);
            self.tfg().graph.link(merge_node, next_node_if_false);

            let checked_stmts = self.check_stmts(block_contents.statements, entry_node, merge_node);

            (
                CheckedType {
                    kind: CheckedTypeKind::Void,
                    span,
                },
                None,
                checked_stmts,
            )
        };

        self.exit_scope();

        CheckedExpr {
            ty: block_type,
            kind: CheckedExprKind::Block(CheckedBlockContents {
                statements: checked_statements,
                final_expr: checked_final_expr,
            }),
        }
    }
}
