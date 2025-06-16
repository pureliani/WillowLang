use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
    usize,
};

use crate::ast::{
    checked::{
        checked_expression::{CheckedExpr, CheckedExprKind, FunctionSummary, RefinementKey},
        checked_type::{CheckedType, CheckedTypeKind},
    },
    DefinitionId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TFGNodeId(usize);

/// Represents the type a specific variable is narrowed to on a given path.
/// This is calculated and stored by the TFG builder.
#[derive(Debug, Clone)]
pub struct NarrowingInfo {
    pub variable: DefinitionId,
    pub narrowed_type: CheckedTypeKind,
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
        target: DefinitionId,
        assigned_type: Rc<CheckedTypeKind>,
        next_node: Option<TFGNodeId>,
    },

    NoOp {
        next_node: Option<TFGNodeId>,
    },
}

pub type TFGNodeVariableTypes = HashMap<DefinitionId, Rc<CheckedTypeKind>>;

#[derive(Debug, Clone)]
pub struct TFGNode {
    pub id: TFGNodeId,
    pub kind: TFGNodeKind,
    pub predecessors: HashSet<TFGNodeId>,
    pub variable_types: TFGNodeVariableTypes,
}

#[derive(Debug, Clone)]
pub struct TypeFlowGraph {
    pub entry_node_id: TFGNodeId,
    nodes: HashMap<TFGNodeId, TFGNode>,
    node_counter: usize,
}

impl TypeFlowGraph {
    pub fn generate_summary(&self) -> FunctionSummary {
        let exit_states = self.analyze_exit_states();
        let guaranteed_calls = self.analyze_guaranteed_calls();

        FunctionSummary {
            exit_states,
            guaranteed_calls,
        }
    }

    fn analyze_exit_states(&self) -> HashMap<RefinementKey, TFGNodeVariableTypes> {
        todo!()
    }

    fn analyze_guaranteed_calls(&self) -> HashSet<DefinitionId> {
        todo!()
    }
}

impl TypeFlowGraph {
    pub fn new() -> Self {
        let entry_node_id = TFGNodeId(0);

        TypeFlowGraph {
            nodes: HashMap::from([(
                entry_node_id,
                TFGNode {
                    variable_types: HashMap::new(),
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
                variable_types: HashMap::new(),
                id,
                kind,
                predecessors: HashSet::new(),
            },
        );

        id
    }

    pub fn link_sequential(&mut self, from_id: TFGNodeId, to_id: TFGNodeId) {
        let from_node = self.nodes.get_mut(&from_id).expect("Expected node with 'from_id' to exist");

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

        let to_node = self.nodes.get_mut(&to_id).expect("Expected node with 'to_id' to exist");

        to_node.predecessors.insert(from_id);
    }

    pub fn link_branch(&mut self, branch_id: TFGNodeId, target_if_true: TFGNodeId, target_if_false: TFGNodeId) {
        let branch_node = self.nodes.get_mut(&branch_id).expect("Branch node doesn't exist");

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
                let intermediate_node_id = self.create_node(TFGNodeKind::NoOp { next_node: None });

                self.build_condition_tfg(&left, prev_node, intermediate_node_id, next_node_if_false);

                self.build_condition_tfg(&right, intermediate_node_id, next_node_if_true, next_node_if_false);
            }
            CheckedExprKind::Or { left, right } => {
                let intermediate_node_id = self.create_node(TFGNodeKind::NoOp { next_node: None });

                self.build_condition_tfg(&left, prev_node, next_node_if_true, intermediate_node_id);

                self.build_condition_tfg(&right, intermediate_node_id, next_node_if_true, next_node_if_false);
            }
            CheckedExprKind::BoolLiteral { value } => {
                let target = if *value { next_node_if_true } else { next_node_if_false };

                // Might need to handle linking from different prev_node types,
                // for now assume prev_node is always linkable sequentially here
                self.link_sequential(prev_node, target);
            }
            _ => {
                let branch_node_id = self.create_node(TFGNodeKind::Branch {
                    narrowing_if_true: None,
                    next_node_if_true: None,
                    narrowing_if_false: None,
                    next_node_if_false: None,
                });

                self.link_sequential(prev_node, branch_node_id);

                if let Some((narrowing_true, narrowing_false)) = analyze_atomic_condition(condition) {
                    let branch_node = self.nodes.get_mut(&branch_node_id).unwrap();
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

                self.link_branch(branch_node_id, next_node_if_true, next_node_if_false);
            }
        }
    }

    pub fn get_node(&self, id: TFGNodeId) -> Option<&TFGNode> {
        self.nodes.get(&id)
    }

    pub fn get_node_mut(&mut self, id: TFGNodeId) -> Option<&mut TFGNode> {
        self.nodes.get_mut(&id)
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = &TFGNode> {
        self.nodes.values()
    }
}

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

fn analyze_atomic_condition(condition: &CheckedExpr) -> Option<(NarrowingInfo, NarrowingInfo)> {
    match &condition.kind {
        CheckedExprKind::IsType { left, target } => {
            if let CheckedExprKind::Identifier(id) = &left.kind {
                if let CheckedTypeKind::Union(types) = &left.ty.kind {
                    let var_id = DefinitionId(id.name.0);

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

        CheckedExprKind::Equal { left, right } | CheckedExprKind::NotEqual { left, right } => {
            let is_not_equal = matches!(condition.kind, CheckedExprKind::NotEqual { .. });

            if let Some((ident_expr, other_expr)) = get_identifier_and_other(&left, &right) {
                if let CheckedExprKind::Identifier(id) = &ident_expr.kind {
                    if let CheckedTypeKind::Union(ident_expr_ty) = &ident_expr.ty.kind {
                        let var_id = DefinitionId(id.name.0);
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
        _ => {}
    }

    None
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
