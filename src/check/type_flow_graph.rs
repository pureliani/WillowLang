use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
    usize,
};

use crate::ast::checked::{
    checked_expression::{CheckedExpr, CheckedExprKind},
    checked_type::CheckedType,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TFGNodeId(usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariableId(String);

/// Represents the type a specific variable is narrowed to on a given path.
/// This is calculated and stored by the TFG builder.
#[derive(Debug, Clone)]
pub struct NarrowingInfo {
    pub variable: VariableId,
    pub narrowed_type: Rc<CheckedType>,
}

/// Represents the different kinds of nodes in the Type Flow Graph.
/// Edges are implicitly defined by the `next_node` fields.
#[derive(Debug, Clone)]
pub enum TFGNodeKind {
    /// Start of the analyzed scope.
    Entry {
        next_node: Option<TFGNodeId>,
    },
    /// End of the analyzed scope (e.g., return, end of block). Terminal node.
    Exit,

    /// Represents a conditional branch point.
    /// The builder determines the immediate type implications for relevant variables.
    Branch {
        /// Info about the primary variable being narrowed if the condition is false.
        /// If the condition itself doesn't narrow (like `x > 0`), the builder stores
        /// None
        narrowing_if_true: Option<NarrowingInfo>,
        next_node_if_true: Option<TFGNodeId>,

        /// Info about the primary variable being narrowed if the condition is false.
        narrowing_if_false: Option<NarrowingInfo>,
        next_node_if_false: Option<TFGNodeId>,
    },

    /// Represents an assignment `target = <expr>`.
    Assign {
        target: VariableId,
        assigned_type: Rc<CheckedType>,
        next_node: Option<TFGNodeId>,
    },

    NoOp {
        next_node: Option<TFGNodeId>,
    },
}

#[derive(Debug)]
pub struct TFGNode {
    pub id: TFGNodeId,
    pub kind: TFGNodeKind,
    pub predecessors: HashSet<TFGNodeId>,
}

#[derive(Debug)]
pub struct TypeFlowGraph {
    nodes: HashMap<TFGNodeId, TFGNode>,
    pub entry_node_id: TFGNodeId,
    node_counter: usize,
}

impl TypeFlowGraph {
    pub fn new() -> Self {
        let entry_node_id = TFGNodeId(0);

        TypeFlowGraph {
            nodes: HashMap::from([(
                entry_node_id,
                TFGNode {
                    id: entry_node_id,
                    kind: TFGNodeKind::Entry { next_node: None },
                    predecessors: HashSet::new(),
                },
            )]),
            entry_node_id,
            node_counter: 1,
        }
    }

    pub fn create_node(&mut self, kind: TFGNodeKind) -> TFGNodeId {
        let id = TFGNodeId(self.node_counter);
        self.node_counter += 1;

        self.nodes.insert(
            id,
            TFGNode {
                id,
                kind,
                predecessors: HashSet::new(),
            },
        );

        id
    }

    fn link_sequential(&mut self, from_id: TFGNodeId, to_id: TFGNodeId) {
        let from_node = self
            .nodes
            .get_mut(&from_id)
            .expect("Expected node with 'from_id' to exist");

        let next_field = match &mut from_node.kind {
            TFGNodeKind::Entry { next_node } => next_node,
            TFGNodeKind::Assign { next_node, .. } => next_node,
            TFGNodeKind::NoOp { next_node } => next_node,
            TFGNodeKind::Branch { .. } => {
                panic!("Cannot link_sequential from a Branch node")
            }
            TFGNodeKind::Exit => panic!("Cannot link_sequential from an Exit node"),
        };

        *next_field = Some(to_id);

        let to_node = self
            .nodes
            .get_mut(&to_id)
            .expect("Expected node with 'to_id' to exist");

        to_node.predecessors.insert(from_id);
    }

    fn link_branch(
        &mut self,
        branch_id: TFGNodeId,
        target_if_true: TFGNodeId,
        target_if_false: TFGNodeId,
    ) {
        let branch_node = self
            .nodes
            .get_mut(&branch_id)
            .expect("Branch node doesn't exist");

        match &mut branch_node.kind {
            TFGNodeKind::Branch {
                next_node_if_true,
                next_node_if_false,
                ..
            } => {
                *next_node_if_true = Some(target_if_true);
                *next_node_if_false = Some(target_if_false);
            }
            _ => panic!("link_branch called on non-branch node"),
        }

        self.nodes
            .get_mut(&target_if_true)
            .expect("target_if_true node must exist")
            .predecessors
            .insert(branch_id);

        self.nodes
            .get_mut(&target_if_false)
            .expect("target_if_false node must exist")
            .predecessors
            .insert(branch_id);
    }

    pub fn get_node(&self, id: TFGNodeId) -> Option<&TFGNode> {
        self.nodes.get(&id)
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = &TFGNode> {
        self.nodes.values()
    }
}

#[derive(Debug, Clone)]
pub struct TypeState {
    pub variable_types: HashMap<VariableId, Rc<CheckedType>>,
}

pub fn union_subtract(
    mut from_union: HashSet<CheckedType>,
    type_to_remove: &CheckedType,
) -> HashSet<CheckedType> {
    match type_to_remove {
        CheckedType::Union(types_to_remove) => {
            for t in types_to_remove {
                from_union.remove(t);
            }
        }
        _ => {
            from_union.remove(type_to_remove);
        }
    };

    from_union
}

fn analyze_atomic_condition(condition: &CheckedExpr) -> Option<(NarrowingInfo, NarrowingInfo)> {
    match &condition.kind {
        CheckedExprKind::IsType { left, target } => {
            if let CheckedExprKind::Identifier(id) = &left.kind {
                let var_id = VariableId(id.name.clone());

                let false_type = if let CheckedType::Union(types) = left.ty.clone() {
                    CheckedType::Union(union_subtract(types, target))
                } else {
                    panic!("Cannot subtract from non-union type")
                };

                let narrowing_true = NarrowingInfo {
                    variable: var_id.clone(),
                    narrowed_type: Rc::new(target.clone()),
                };
                let narrowing_false = NarrowingInfo {
                    variable: var_id,
                    narrowed_type: Rc::new(false_type),
                };
                return Some((narrowing_true, narrowing_false));
            }
        }

        CheckedExprKind::NotEqual { left, right } => {
            let ident_expr = if matches!(right.kind, CheckedExprKind::Null) {
                Some(left)
            } else if matches!(left.kind, CheckedExprKind::Null) {
                Some(right)
            } else {
                None
            };

            if let Some(expr) = ident_expr {
                if let CheckedExprKind::Identifier(id) = &expr.kind {
                    let var_id = VariableId(id.name.clone());

                    let true_type = if let CheckedType::Union(types) = left.ty.clone() {
                        CheckedType::Union(union_subtract(types, &CheckedType::Null))
                    } else {
                        panic!("Cannot subtract from non-union type")
                    };

                    let narrowing_true = NarrowingInfo {
                        variable: var_id.clone(),
                        narrowed_type: Rc::new(true_type),
                    };
                    let narrowing_false = NarrowingInfo {
                        variable: var_id,
                        narrowed_type: Rc::new(CheckedType::Null),
                    };
                    return Some((narrowing_true, narrowing_false));
                }
            }
        }

        CheckedExprKind::Equal { left, right } => {
            let ident_expr = if matches!(right.kind, CheckedExprKind::Null) {
                Some(left)
            } else if matches!(left.kind, CheckedExprKind::Null) {
                Some(right)
            } else {
                None
            };

            if let Some(expr) = ident_expr {
                if let CheckedExprKind::Identifier(id) = &expr.kind {
                    let var_id = VariableId(id.name.clone());
                    let null_type = Rc::new(CheckedType::Null);

                    let false_type = if let CheckedType::Union(types) = left.ty.clone() {
                        CheckedType::Union(union_subtract(types, &CheckedType::Null))
                    } else {
                        panic!("Cannot subtract from non-union type")
                    };

                    let narrowing_true = NarrowingInfo {
                        variable: var_id.clone(),
                        narrowed_type: null_type,
                    };
                    let narrowing_false = NarrowingInfo {
                        variable: var_id,
                        narrowed_type: Rc::new(false_type),
                    };
                    return Some((narrowing_true, narrowing_false));
                }
            }
        }
        _ => {}
    }

    None
}

fn build_condition_tfg(
    tfg: &mut TypeFlowGraph,
    condition: &CheckedExpr,
    prev_node: TFGNodeId,
    next_node_if_true: TFGNodeId,
    next_node_if_false: TFGNodeId,
) {
    match &condition.kind {
        CheckedExprKind::And { left, right } => {
            let intermediate_node_id = tfg.create_node(TFGNodeKind::NoOp { next_node: None });

            build_condition_tfg(
                tfg,
                &left,
                prev_node,
                intermediate_node_id,
                next_node_if_false,
            );

            build_condition_tfg(
                tfg,
                &right,
                intermediate_node_id,
                next_node_if_true,
                next_node_if_false,
            );
        }
        CheckedExprKind::Or { left, right } => {
            let intermediate_node_id = tfg.create_node(TFGNodeKind::NoOp { next_node: None });

            build_condition_tfg(
                tfg,
                &left,
                prev_node,
                next_node_if_true,
                intermediate_node_id,
            );

            build_condition_tfg(
                tfg,
                &right,
                intermediate_node_id,
                next_node_if_true,
                next_node_if_false,
            );
        }
        CheckedExprKind::BoolLiteral { value } => {
            let target = if *value {
                next_node_if_true
            } else {
                next_node_if_false
            };

            // Need to handle linking from different prev_node types
            // For simplicity, assume prev_node is always linkable sequentially here,
            // but a robust implementation might need checks or different linking.
            tfg.link_sequential(prev_node, target);
        }

        _ => {
            let branch_node_id = tfg.create_node(TFGNodeKind::Branch {
                narrowing_if_true: None,
                next_node_if_true: None,
                narrowing_if_false: None,
                next_node_if_false: None,
            });

            // Link the previous node to this new branch
            // Assume prev_node is suitable for link_sequential. If prev_node could be
            // a Branch itself, this linking needs adjustment. In the current recursive
            // structure, prev_node will be Entry, NoOp, or another Branch's target.
            // Let's assume link_sequential works for Entry/NoOp.
            tfg.link_sequential(prev_node, branch_node_id);

            if let Some((narrowing_true, narrowing_false)) = analyze_atomic_condition(condition) {
                let branch_node = tfg.nodes.get_mut(&branch_node_id).unwrap();
                if let TFGNodeKind::Branch {
                    narrowing_if_true: nit,
                    narrowing_if_false: nif,
                    ..
                } = &mut branch_node.kind
                {
                    *nit = Some(narrowing_true);
                    *nif = Some(narrowing_false);
                }
            }

            tfg.link_branch(branch_node_id, next_node_if_true, next_node_if_false);
        }
    }
}
