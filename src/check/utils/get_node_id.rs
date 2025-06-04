use crate::{ast::NodeId, check::SemanticChecker};

impl<'a> SemanticChecker<'a> {
    pub fn get_node_id(&mut self) -> NodeId {
        let value = self.node_counter;
        self.node_counter += 1;
        NodeId(value)
    }
}
