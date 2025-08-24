use crate::{cfg::Instruction, hir_builder::HIRBuilder};

impl<'a> HIRBuilder<'a> {
    pub fn add_basic_block_instruction(&mut self, instruction: Instruction) {
        let current_basic_block = self.cfg.blocks.get_mut(&self.current_block_id);

        if let Some(bb) = current_basic_block {
            bb.instructions.push(instruction);
        } else {
            panic!(
                "Could not add an instruction to a basic block: basic block with id: {} doesn't exist.",
                self.current_block_id.0
            );
        }
    }
}
