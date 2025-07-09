use crate::{ast::DefinitionId, hir_builder::HIRBuilder};

impl<'a> HIRBuilder<'a> {
    pub fn get_definition_id(&mut self) -> DefinitionId {
        let val = DefinitionId(self.definition_counter);
        self.definition_counter += 1;
        val
    }
}
