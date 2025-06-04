use std::collections::HashMap;

use crate::ast::{NodeId, Span};

#[derive(Debug, Clone)]
pub struct SpanRegistry {
    registry: HashMap<NodeId, Span>,
    id_counter: usize,
}

impl SpanRegistry {
    pub fn new() -> Self {
        Self {
            id_counter: 0,
            registry: HashMap::new(),
        }
    }

    pub fn get_span(&self, id: NodeId) -> &Span {
        self.registry
            .get(&id)
            .expect("Expected the ast node with id \"{}\" to have an associated span")
    }

    pub fn add_span(&mut self, span: Span) {
        self.registry.insert(NodeId(self.id_counter), span);
        self.id_counter += 1;
    }
}
