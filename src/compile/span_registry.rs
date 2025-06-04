use std::collections::HashMap;

use crate::ast::{NodeId, Span};

#[derive(Debug, Clone)]
pub struct SpanRegistry {
    registry: HashMap<NodeId, Span>,
}

impl SpanRegistry {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
        }
    }

    pub fn get_span(&self, id: NodeId) -> &Span {
        self.registry
            .get(&id)
            .expect("Expected the ast node with id \"{}\" to have an associated span")
    }

    pub fn insert_span(&mut self, id: NodeId, span: Span) {
        self.registry.insert(id, span);
    }
}
