use crate::{cfg::ValueId, hir_builder::HIRBuilder};

impl<'a> HIRBuilder<'a> {
    pub fn new_value_id(&mut self) -> ValueId {
        let id = ValueId(self.value_id_counter);
        self.value_id_counter += 1;
        id
    }
}
