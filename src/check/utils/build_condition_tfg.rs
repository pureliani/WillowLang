use std::collections::HashSet;

use crate::{
    ast::checked::{
        checked_expression::{CheckedExpr, CheckedExprKind},
        checked_type::{CheckedType, CheckedTypeKind},
    },
    check::{utils::scope::SymbolEntry, SemanticChecker},
    tfg::{NarrowingInfo, TFGNodeId, TFGNodeKind},
};

pub fn union_subtract(from_union: &HashSet<CheckedType>, type_to_remove: &CheckedType) -> HashSet<CheckedType> {
    let mut result = from_union.clone();

    match &type_to_remove.kind {
        CheckedTypeKind::Union(types_to_remove) => {
            for t in types_to_remove {
                result.remove(t);
            }
        }
        _ => {
            result.remove(type_to_remove);
        }
    };

    result
}

fn get_identifier_and_other<'a>(left: &'a CheckedExpr, right: &'a CheckedExpr) -> Option<(&'a CheckedExpr, &'a CheckedExpr)> {
    if matches!(left.kind, CheckedExprKind::Identifier(..)) {
        Some((left, right))
    } else if matches!(right.kind, CheckedExprKind::Identifier(..)) {
        Some((right, left))
    } else {
        None
    }
}

impl<'a> SemanticChecker<'a> {
    pub fn analyze_atomic_condition(&self, condition: &CheckedExpr) -> Option<(NarrowingInfo, NarrowingInfo)> {
        match &condition.kind {
            CheckedExprKind::IsType { left, target } => {
                if let CheckedExprKind::Identifier(id) = &left.kind {
                    if let Some(SymbolEntry::VarDecl(decl)) = self.scope_lookup(id.name) {
                        let var_id = decl.borrow().id;
                        if let CheckedTypeKind::Union(types) = &left.ty.kind {
                            let subtracted = CheckedTypeKind::Union(union_subtract(types, target));

                            let narrowing_true = NarrowingInfo {
                                variable: var_id.clone(),
                                narrowed_type: target.kind.clone(),
                            };

                            let narrowing_false = NarrowingInfo {
                                variable: var_id,
                                narrowed_type: subtracted,
                            };

                            return Some((narrowing_true, narrowing_false));
                        };
                    }
                }
            }

            CheckedExprKind::Equal { left, right } | CheckedExprKind::NotEqual { left, right } => {
                let is_not_equal = matches!(condition.kind, CheckedExprKind::NotEqual { .. });

                if let Some((ident_expr, other_expr)) = get_identifier_and_other(&left, &right) {
                    if let CheckedExprKind::Identifier(id) = &ident_expr.kind {
                        if let Some(SymbolEntry::VarDecl(decl)) = self.scope_lookup(id.name) {
                            let var_id = decl.borrow().id;
                            if let CheckedTypeKind::Union(ident_expr_ty) = &ident_expr.ty.kind {
                                let narrow_to_type = &other_expr.ty;

                                let subtracted = CheckedTypeKind::Union(union_subtract(ident_expr_ty, narrow_to_type));

                                let narrowing_to_specific = NarrowingInfo {
                                    variable: var_id.clone(),
                                    narrowed_type: narrow_to_type.kind.clone(),
                                };

                                let narrowing_to_subtracted = NarrowingInfo {
                                    variable: var_id,
                                    narrowed_type: subtracted,
                                };

                                if is_not_equal {
                                    return Some((narrowing_to_subtracted, narrowing_to_specific));
                                } else {
                                    return Some((narrowing_to_specific, narrowing_to_subtracted));
                                }
                            };
                        }
                    }
                }
            }
            _ => {}
        }

        None
    }

    pub fn build_condition_tfg(
        &mut self,
        condition: &CheckedExpr,
        prev_node: TFGNodeId,
        next_node_if_true: TFGNodeId,
        next_node_if_false: TFGNodeId,
    ) {
        match &condition.kind {
            CheckedExprKind::Not { right } => {
                self.build_condition_tfg(right, prev_node, next_node_if_false, next_node_if_true);
            }
            CheckedExprKind::And { left, right } => {
                let intermediate_node_id = self
                    .tfg_contexts
                    .last_mut()
                    .unwrap()
                    .graph
                    .create_node(TFGNodeKind::NoOp { next_node: None });

                self.build_condition_tfg(left, prev_node, intermediate_node_id, next_node_if_false);
                self.build_condition_tfg(right, intermediate_node_id, next_node_if_true, next_node_if_false);
            }
            CheckedExprKind::Or { left, right } => {
                let intermediate_node_id = self
                    .tfg_contexts
                    .last_mut()
                    .unwrap()
                    .graph
                    .create_node(TFGNodeKind::NoOp { next_node: None });

                self.build_condition_tfg(left, prev_node, next_node_if_true, intermediate_node_id);
                self.build_condition_tfg(right, intermediate_node_id, next_node_if_true, next_node_if_false);
            }
            CheckedExprKind::BoolLiteral { value } => {
                let target = if *value { next_node_if_true } else { next_node_if_false };
                self.tfg_contexts.last_mut().unwrap().graph.link_successor(prev_node, target);
            }
            _ => {
                let (narrowing_if_true, narrowing_if_false) = self
                    .analyze_atomic_condition(condition)
                    .map(|(t, f)| (Some(t), Some(f)))
                    .unwrap_or((None, None));

                let ctx = self.tfg_contexts.last_mut().unwrap();

                let branch_node_id = ctx.graph.create_node(TFGNodeKind::Branch {
                    narrowing_if_true,
                    next_node_if_true: None,
                    narrowing_if_false,
                    next_node_if_false: None,
                });

                ctx.graph.link_successor(prev_node, branch_node_id);

                ctx.graph.link_branch(branch_node_id, next_node_if_true, next_node_if_false);

                ctx.graph.apply_narrowing(branch_node_id, next_node_if_true, true);
                ctx.graph.apply_narrowing(branch_node_id, next_node_if_false, false);
            }
        }
    }
}
