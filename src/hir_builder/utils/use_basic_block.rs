use crate::{cfg::BasicBlockId, hir_builder::HIRBuilder};

impl<'a> HIRBuilder<'a> {
    pub fn use_basic_block(&mut self, id: BasicBlockId) {
        if let Some(_) = self.cfg.blocks.get(&id) {
            self.current_block_id = id;
        } else {
            panic!("Could not use basic block with id {} as it doesn't exist", id.0);
        }
    }
}
