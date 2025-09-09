use crate::{cfg::HeapAllocationId, hir_builder::FunctionBuilder};

impl FunctionBuilder {
    pub fn new_allocation_id(&mut self) -> HeapAllocationId {
        let id = HeapAllocationId(self.allocation_id_counter);
        self.allocation_id_counter += 1;
        id
    }
}
