use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
    usize,
};

use crate::{
    ast::{
        checked::{
            checked_expression::{FunctionSummary, RefinementKey},
            checked_type::CheckedTypeKind,
        },
        DefinitionId,
    },
    check::utils::union_of::union_of_kinds,
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
        // TODO: Implement this function
        let dummy_exit_states = HashMap::new();
        dummy_exit_states
    }

    fn analyze_guaranteed_calls(&self) -> HashSet<DefinitionId> {
        // TODO: Implement this function
        let dummy_guaranteed_calls = HashSet::new();
        dummy_guaranteed_calls
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

    pub fn create_merge_node(&mut self, parent_ids: &[TFGNodeId]) -> TFGNodeId {
        if parent_ids.is_empty() {
            panic!("create_merge_node expected parent_ids to not be empty")
        }

        if parent_ids.len() == 1 {
            return parent_ids[0];
        }

        let merge_node_id = self.create_node(TFGNodeKind::NoOp { next_node: None });

        let mut all_vars: HashSet<DefinitionId> = HashSet::new();
        let mut parent_type_maps: Vec<&TFGNodeVariableTypes> = Vec::new();

        for parent_id in parent_ids {
            if let Some(parent_node) = self.nodes.get(parent_id) {
                all_vars.extend(parent_node.variable_types.keys());
                parent_type_maps.push(&parent_node.variable_types);
            }
        }

        let mut merged_variable_types: TFGNodeVariableTypes = HashMap::new();

        for var_id in all_vars {
            let types_for_var: Vec<Rc<CheckedTypeKind>> = parent_type_maps
                .iter()
                .filter_map(|type_map| type_map.get(&var_id).cloned())
                .collect();

            if !types_for_var.is_empty() {
                let unioned_type = union_of_kinds(types_for_var);
                merged_variable_types.insert(var_id, unioned_type);
            }
        }

        let merge_node = self.nodes.get_mut(&merge_node_id).unwrap();
        merge_node.variable_types = merged_variable_types;

        for parent_id in parent_ids {
            self.link_successor(*parent_id, merge_node_id);
        }

        merge_node_id
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

    pub fn link_successor(&mut self, from_id: TFGNodeId, to_id: TFGNodeId) {
        let from_node = self.nodes.get_mut(&from_id).expect("from_id must exist in link_successor");

        match &mut from_node.kind {
            TFGNodeKind::Entry { next_node } => *next_node = Some(to_id),
            TFGNodeKind::Assign { next_node, .. } => *next_node = Some(to_id),
            TFGNodeKind::NoOp { next_node } => *next_node = Some(to_id),
            TFGNodeKind::Branch {
                next_node_if_true,
                next_node_if_false,
                ..
            } => {
                if next_node_if_false.is_none() {
                    *next_node_if_false = Some(to_id);
                } else if next_node_if_true.is_none() {
                    *next_node_if_true = Some(to_id);
                } else {
                    // If both paths are already linked, trying to link this node again
                    // might be a logic error in the semantic checker.
                }
            }
            TFGNodeKind::Exit => {}
        }

        if !matches!(self.nodes.get(&from_id).unwrap().kind, TFGNodeKind::Exit) {
            let to_node = self.nodes.get_mut(&to_id).expect("to_id must exist in link_successor");
            to_node.predecessors.insert(from_id);
        }
    }

    pub fn apply_narrowing(&mut self, branch_id: TFGNodeId, target_id: TFGNodeId, is_true_path: bool) {
        let branch_node = self.nodes.get(&branch_id).unwrap().clone();

        let parent_types = self
            .nodes
            .get(&branch_node.predecessors.iter().next().unwrap())
            .unwrap()
            .variable_types
            .clone();

        let target_node = self.nodes.get_mut(&target_id).unwrap();
        target_node.variable_types = parent_types;

        if let TFGNodeKind::Branch {
            narrowing_if_true,
            narrowing_if_false,
            ..
        } = branch_node.kind
        {
            let narrowing_info = if is_true_path { narrowing_if_true } else { narrowing_if_false };
            if let Some(info) = narrowing_info {
                target_node.variable_types.insert(info.variable, Rc::new(info.narrowed_type));
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
