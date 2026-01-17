use std::collections::HashMap;

use crate::hir::{cfg::BasicBlock, counters::next_block_id, FunctionBuilder};

impl FunctionBuilder {
    pub fn new_basic_block(&mut self) -> &mut BasicBlock {
        let id = next_block_id();
        self.cfg.blocks.entry(id).or_insert(BasicBlock {
            id,
            original_to_local_valueid: HashMap::new(),
            predecessors: vec![],
            instructions: vec![],
            terminator: None,
            params: vec![],
            incomplete_params: vec![],
            sealed: false,
        })
    }
}
