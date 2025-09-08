use crate::{cfg::Terminator, hir_builder::FunctionBuilder};

impl<'a> FunctionBuilder<'a> {
    pub fn set_basic_block_terminator(&mut self, terminator: Terminator) {
        let current_basic_block = self.cfg.blocks.get_mut(&self.current_block_id);

        if let Some(bb) = current_basic_block {
            bb.terminator = terminator;
        } else {
            panic!(
                "Could not set basic block terminator: basic block with id: {} doesn't exist.",
                self.current_block_id.0
            );
        }
    }
}
