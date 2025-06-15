use crate::{ast::DefinitionId, check::SemanticChecker};

impl<'a> SemanticChecker<'a> {
    pub fn get_definition_id(&mut self) -> DefinitionId {
        let val = DefinitionId(self.definition_counter);
        self.definition_counter += 1;
        val
    }
}
