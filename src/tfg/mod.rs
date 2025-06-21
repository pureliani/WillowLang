use std::{
    collections::{HashMap, HashSet},
    usize,
};

use crate::ast::{
    checked::{
        checked_expression::{FunctionSummary, RefinementKey},
        checked_type::CheckedTypeKind,
    },
    DefinitionId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TFGNodeId(usize);

#[derive(Debug, Clone)]
pub struct NarrowingInfo {
    pub target: DefinitionId,
    pub narrowed_type: CheckedTypeKind,
}

#[derive(Debug, Clone)]
pub enum TFGNodeKind {
    Entry {
        next_node: Option<TFGNodeId>,
    },
    Narrowing {
        info: NarrowingInfo,
        next_node: Option<TFGNodeId>,
    },
    BranchNarrowing {
        narrowing_if_true: Option<NarrowingInfo>,
        next_node_if_true: Option<TFGNodeId>,
        narrowing_if_false: Option<NarrowingInfo>,
        next_node_if_false: Option<TFGNodeId>,
    },
    NoOp {
        next_node: Option<TFGNodeId>,
    },
    Exit,
}

pub type TFGNodeVariableTypes = HashMap<DefinitionId, CheckedTypeKind>;

#[derive(Debug, Clone)]
pub struct TFGNode {
    pub id: TFGNodeId,
    pub kind: TFGNodeKind,
    pub predecessors: HashSet<TFGNodeId>,
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

    pub fn link_sequential(&mut self, from_id: TFGNodeId, to_id: TFGNodeId) {
        let from_node = self.nodes.get_mut(&from_id).expect("Expected node with 'from_id' to exist");

        let next_field = match &mut from_node.kind {
            TFGNodeKind::Entry { next_node } => next_node,
            TFGNodeKind::Narrowing { next_node, .. } => next_node,
            TFGNodeKind::NoOp { next_node } => next_node,
            TFGNodeKind::BranchNarrowing { .. } => {
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
            TFGNodeKind::BranchNarrowing {
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
            TFGNodeKind::Narrowing { next_node, .. } => *next_node = Some(to_id),
            TFGNodeKind::NoOp { next_node } => *next_node = Some(to_id),
            TFGNodeKind::BranchNarrowing {
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
