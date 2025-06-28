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
        true_continuation: TFGNodeId,
        false_continuation: TFGNodeId,
    ) -> CheckedExpr {
        self.enter_scope(ScopeKind::CodeBlock);
        self.placeholder_declarations(&block_contents.statements);

        let mut current_node = entry_node;

        let mut checked_statements = Vec::new();
        for stmt in block_contents.statements {
            let next_node = self.tfg().graph.create_node(TFGNodeKind::NoOp);
            let checked_stmt = self.check_stmt(stmt, current_node, next_node);
            checked_statements.push(checked_stmt);
            current_node = next_node;
        }

        let (block_type, checked_final_expr) = if let Some(final_expr) = block_contents.final_expr {
            let checked_expr = self.check_expr(*final_expr, current_node, true_continuation, false_continuation);
            (checked_expr.ty.clone(), Some(Box::new(checked_expr)))
        } else {
            self.tfg().graph.link(current_node, true_continuation);
            self.tfg().graph.link(current_node, false_continuation);
            (
                CheckedType {
                    kind: CheckedTypeKind::Void,
                    span,
                },
                None,
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
