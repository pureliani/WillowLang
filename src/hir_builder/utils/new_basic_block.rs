use crate::{
    cfg::{BasicBlock, BasicBlockId, Terminator},
    hir_builder::FunctionBuilder,
};

impl<'a> FunctionBuilder<'a> {
    pub fn new_basic_block(&mut self) -> BasicBlockId {
        let block_id = BasicBlockId(self.block_id_counter);
        self.block_id_counter += 1;
        self.cfg.blocks.insert(
            block_id,
            BasicBlock {
                id: block_id,
                instructions: vec![],
                terminator: Terminator::Unreachable,
            },
        );

        block_id
    }
}
