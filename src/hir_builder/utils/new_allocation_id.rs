use crate::{cfg::HeapAllocationId, hir_builder::FunctionBuilder};

impl<'a> FunctionBuilder<'a> {
    pub fn new_allocation_id(&mut self) -> HeapAllocationId {
        let id = HeapAllocationId(self.allocation_counter);
        self.allocation_counter += 1;
        id
    }
}
