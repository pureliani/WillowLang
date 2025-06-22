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

        let then_entry_node = self
            .tfg_contexts
            .last_mut()
            .unwrap()
            .graph
            .create_node(TFGNodeKind::NoOp { next_node: None });
        let mut false_path_entry_node = self
            .tfg_contexts
            .last_mut()
            .unwrap()
            .graph
            .create_node(TFGNodeKind::NoOp { next_node: None });

        let checked_condition = self.check_condition_expr(*condition, then_entry_node, false_path_entry_node);

        self.tfg_contexts.last_mut().unwrap().current_node = then_entry_node;
        let (then_type, checked_then_branch) = self.check_codeblock(then_branch);
        let then_final_node = self.tfg_contexts.last().unwrap().current_node;

        let mut checked_else_if_branches = Vec::new();
        let mut else_if_final_nodes_and_types: Vec<(TFGNodeId, CheckedType)> = Vec::new();

        for (elseif_cond_expr, elseif_block) in else_if_branches {
            self.tfg_contexts.last_mut().unwrap().current_node = false_path_entry_node;

            let elseif_then_entry = self
                .tfg_contexts
                .last_mut()
                .unwrap()
                .graph
                .create_node(TFGNodeKind::NoOp { next_node: None });
            let next_false_path_entry = self
                .tfg_contexts
                .last_mut()
                .unwrap()
                .graph
                .create_node(TFGNodeKind::NoOp { next_node: None });

            let checked_elseif_condition = self.check_condition_expr(*elseif_cond_expr, elseif_then_entry, next_false_path_entry);

            self.tfg_contexts.last_mut().unwrap().current_node = elseif_then_entry;
            let (codeblock_type, checked_codeblock) = self.check_codeblock(elseif_block);
            let final_node_for_this_elseif = self.tfg_contexts.last().unwrap().current_node;

            checked_else_if_branches.push((Box::new(checked_elseif_condition), checked_codeblock));
            else_if_final_nodes_and_types.push((final_node_for_this_elseif, codeblock_type));

            false_path_entry_node = next_false_path_entry;
        }

        let mut else_final_node_and_type: Option<(TFGNodeId, CheckedType)> = None;
        let checked_else_branch = else_branch.map(|br| {
            self.tfg_contexts.last_mut().unwrap().current_node = false_path_entry_node;
            let (else_type, checked_block) = self.check_codeblock(br);
            let final_node_for_else = self.tfg_contexts.last().unwrap().current_node;
            else_final_node_and_type = Some((final_node_for_else, else_type));
            checked_block
        });

        let mut all_branch_final_nodes: Vec<TFGNodeId> = vec![then_final_node];
        let mut all_branch_types: Vec<CheckedType> = vec![then_type];

        for (node, ty) in else_if_final_nodes_and_types {
            all_branch_final_nodes.push(node);
            all_branch_types.push(ty);
        }

        if let Some((node, ty)) = else_final_node_and_type {
            all_branch_final_nodes.push(node);
            all_branch_types.push(ty);
        } else {
            all_branch_final_nodes.push(false_path_entry_node);
            all_branch_types.push(CheckedType {
                kind: CheckedTypeKind::Void,
                span,
            });
        }

        let ctx = self.tfg_contexts.last_mut().unwrap();
        let merge_node_id = ctx.graph.create_node(TFGNodeKind::NoOp { next_node: None });

        for final_node_id in all_branch_final_nodes {
            if let Some(exit_node_data) = ctx.graph.get_node(final_node_id) {
                if !matches!(exit_node_data.kind, TFGNodeKind::Exit) {
                    ctx.graph.link_successor(final_node_id, merge_node_id);
                }
            }
        }
        ctx.current_node = merge_node_id;

        let expr_type = union_of(all_branch_types, span);

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
