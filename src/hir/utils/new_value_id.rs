use crate::hir::{cfg::ValueId, FunctionBuilder};

impl FunctionBuilder {
    pub fn new_value_id(&mut self) -> ValueId {
        let id = ValueId(self.value_id_counter);
        self.value_id_counter += 1;
        id
    }
}
