use crate::{cfg::Instruction, hir_builder::FunctionBuilder};

impl FunctionBuilder {
    pub fn add_basic_block_instruction(&mut self, instruction: Instruction) {
        let bb = self.get_current_basic_block();
        bb.instructions.push(instruction);
    }
}
