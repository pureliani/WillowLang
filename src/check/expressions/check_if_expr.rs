use crate::{
    ast::{
        base::base_expression::{BlockContents, Expr},
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
        Span,
    },
    check::{utils::union_of::union_of, SemanticChecker},
    tfg::{TFGNodeId, TFGNodeKind},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_if_expr(
        &mut self,
        condition: Box<Expr>,
        then_branch: BlockContents,
        else_if_branches: Vec<(Box<Expr>, BlockContents)>,
        else_branch: Option<BlockContents>,
        span: Span,
    ) -> CheckedExpr {
        if self.tfg_contexts.is_empty() {
            panic!("'if' expression checked outside of a function context.");
        }

        let ctx = self.tfg_contexts.last_mut().unwrap();
        let then_entry_node = ctx.graph.create_node(TFGNodeKind::NoOp { next_node: None });
        let mut false_path_node = ctx.graph.create_node(TFGNodeKind::NoOp { next_node: None });

        let checked_condition = self.check_condition_expr(*condition, then_entry_node, false_path_node);

        let ctx = self.tfg_contexts.last_mut().unwrap();
        ctx.current_node = then_entry_node;
        let (then_type, checked_then_branch) = self.check_codeblock(then_branch);

        let ctx = self.tfg_contexts.last_mut().unwrap();
        let then_final_node = ctx.current_node;

        let mut checked_else_if_branches = Vec::new();
        let mut else_if_final_nodes: Vec<(TFGNodeId, CheckedType)> = Vec::new();

        for (elseif_cond_expr, elseif_block) in else_if_branches {
            let ctx = self.tfg_contexts.last_mut().unwrap();
            ctx.current_node = false_path_node;

            let then_entry = ctx.graph.create_node(TFGNodeKind::NoOp { next_node: None });
            let false_path = ctx.graph.create_node(TFGNodeKind::NoOp { next_node: None });

            let checked_condition = self.check_condition_expr(*elseif_cond_expr, then_entry, false_path);

            let ctx = self.tfg_contexts.last_mut().unwrap();
            ctx.current_node = then_entry;

            let (codeblock_type, checked_codeblock) = self.check_codeblock(elseif_block);
            let final_node = self.tfg_contexts.last().unwrap().current_node;

            checked_else_if_branches.push((Box::new(checked_condition), checked_codeblock));
            else_if_final_nodes.push((final_node, codeblock_type));

            false_path_node = false_path;
        }

        let mut else_final_node: Option<(TFGNodeId, CheckedType)> = None;
        let checked_else_branch = else_branch.map(|br| {
            self.tfg_contexts.last_mut().unwrap().current_node = false_path_node;
            let (else_type, checked_block) = self.check_codeblock(br);
            let final_node = self.tfg_contexts.last().unwrap().current_node;
            else_final_node = Some((final_node, else_type));
            checked_block
        });

        let mut final_path_nodes: Vec<TFGNodeId> = vec![then_final_node];
        let mut final_branch_types: Vec<CheckedType> = vec![then_type];

        for (node, ty) in else_if_final_nodes {
            final_path_nodes.push(node);
            final_branch_types.push(ty);
        }

        if let Some((node, ty)) = else_final_node {
            final_path_nodes.push(node);
            final_branch_types.push(ty);
        } else {
            final_path_nodes.push(false_path_node);
            final_branch_types.push(CheckedType {
                kind: CheckedTypeKind::Void,
                span,
            });
        }

        let expr_type = union_of(final_branch_types, span);

        CheckedExpr {
            ty: expr_type,
            kind: CheckedExprKind::If {
                condition: Box::new(checked_condition),
                then_branch: checked_then_branch,
                else_if_branches: checked_else_if_branches,
                else_branch: checked_else_branch,
            },
        }
    }
}
