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
    Entry,
    Narrowing(NarrowingInfo),
    NoOp,
    Exit,
}

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

pub type TFGNodeVariableTypes = HashMap<DefinitionId, CheckedTypeKind>;

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
                    kind: TFGNodeKind::Entry,
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

    pub fn link(&mut self, from_id: TFGNodeId, to_id: TFGNodeId) {
        let from_node = self.nodes.get(&from_id).expect("Expected node with 'from_id' to exist");

        if let TFGNodeKind::Exit = from_node.kind {
            panic!("Cannot link from an Exit node")
        };

        let to_node = self.nodes.get_mut(&to_id).expect("Expected node with 'to_id' to exist");

        to_node.predecessors.insert(from_id);
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
