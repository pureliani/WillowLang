use crate::hir::{
    cfg::{BasicBlock, BasicBlockId},
    FunctionBuilder,
};

impl FunctionBuilder {
    pub fn new_basic_block(&mut self) -> BasicBlockId {
        let block_id = BasicBlockId(self.block_id_counter);
        self.block_id_counter += 1;
        self.cfg.blocks.insert(
            block_id,
            BasicBlock {
                id: block_id,
                instructions: vec![],
                terminator: None,
                phis: vec![],
            },
        );

        block_id
    }
}
