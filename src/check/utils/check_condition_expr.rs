use std::collections::HashSet;

use crate::{
    ast::{
        base::base_expression::{Expr, ExprKind},
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
    },
    check::{utils::scope::SymbolEntry, SemanticChecker, SemanticError},
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
                                target: var_id.clone(),
                                narrowed_type: target.kind.clone(),
                            };

                            let narrowing_false = NarrowingInfo {
                                target: var_id,
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
                                    target: var_id.clone(),
                                    narrowed_type: narrow_to_type.kind.clone(),
                                };

                                let narrowing_to_subtracted = NarrowingInfo {
                                    target: var_id,
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

    pub fn check_condition_expr(&mut self, condition: Expr, true_target: TFGNodeId, false_target: TFGNodeId) -> CheckedExpr {
        let prev_node = self.tfg_contexts.last().unwrap().current_node;
        let condition_span = condition.span;

        match condition.kind {
            ExprKind::Not { right } => {
                let checked_right = self.check_condition_expr(*right, false_target, true_target);
                CheckedExpr {
                    ty: CheckedType {
                        kind: CheckedTypeKind::Bool,
                        span: condition_span,
                    },
                    kind: CheckedExprKind::Not {
                        right: Box::new(checked_right),
                    },
                }
            }
            ExprKind::And { left, right } => {
                let intermediate_node = self
                    .tfg_contexts
                    .last_mut()
                    .unwrap()
                    .graph
                    .create_node(TFGNodeKind::NoOp { next_node: None });
                let checked_left = self.check_condition_expr(*left, intermediate_node, false_target);

                self.tfg_contexts.last_mut().unwrap().current_node = intermediate_node;
                let checked_right = self.check_condition_expr(*right, true_target, false_target);

                CheckedExpr {
                    ty: CheckedType {
                        kind: CheckedTypeKind::Bool,
                        span: condition_span,
                    },
                    kind: CheckedExprKind::And {
                        left: Box::new(checked_left),
                        right: Box::new(checked_right),
                    },
                }
            }
            ExprKind::Or { left, right } => {
                let intermediate_node = self
                    .tfg_contexts
                    .last_mut()
                    .unwrap()
                    .graph
                    .create_node(TFGNodeKind::NoOp { next_node: None });

                let checked_left = self.check_condition_expr(*left, true_target, intermediate_node);

                self.tfg_contexts.last_mut().unwrap().current_node = intermediate_node;
                let checked_right = self.check_condition_expr(*right, true_target, false_target);

                CheckedExpr {
                    ty: CheckedType {
                        kind: CheckedTypeKind::Bool,
                        span: condition_span,
                    },
                    kind: CheckedExprKind::Or {
                        left: Box::new(checked_left),
                        right: Box::new(checked_right),
                    },
                }
            }
            ExprKind::BoolLiteral { value } => {
                let target = if value { true_target } else { false_target };
                self.tfg_contexts.last_mut().unwrap().graph.link_successor(prev_node, target);

                CheckedExpr {
                    kind: CheckedExprKind::BoolLiteral { value },
                    ty: CheckedType {
                        kind: CheckedTypeKind::Bool,
                        span: condition_span,
                    },
                }
            }
            _ => {
                let checked_expr = self.check_expr(condition);

                let expected_bool = CheckedType {
                    kind: CheckedTypeKind::Bool,
                    span: checked_expr.ty.span,
                };
                if !self.check_is_assignable(&checked_expr.ty, &expected_bool) {
                    self.errors.push(SemanticError::TypeMismatch {
                        expected: expected_bool,
                        received: checked_expr.ty.clone(),
                    });
                }

                let (narrowing_if_true, narrowing_if_false) = self
                    .analyze_atomic_condition(&checked_expr)
                    .map(|(t, f)| (Some(t), Some(f)))
                    .unwrap_or((None, None));

                let ctx = self.tfg_contexts.last_mut().unwrap();

                let branch_node_id = ctx.graph.create_node(TFGNodeKind::BranchNarrowing {
                    narrowing_if_true,
                    next_node_if_true: None,
                    narrowing_if_false,
                    next_node_if_false: None,
                });

                ctx.graph.link_successor(prev_node, branch_node_id);
                ctx.graph.link_branch(branch_node_id, true_target, false_target);

                checked_expr
            }
        }
    }
}
