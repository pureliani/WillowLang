use crate::hir::{
    cfg::{BasicBlock, BasicBlockId, Terminator, Value, ValueId},
    errors::SemanticError,
    types::checked_type::Type,
    utils::try_unify_types::try_unify_types,
    FunctionBuilder, HIRContext,
};

impl FunctionBuilder {
    pub fn use_basic_block(&mut self, id: BasicBlockId) {
        if let Some(_) = self.cfg.blocks.get(&id) {
            self.current_block_id = id;
        } else {
            panic!(
                "INTERNAL COMPILER ERROR: Could not use basic block with id {} as it \
                 doesn't exist",
                id.0
            );
        }
    }

    pub fn alloc_value(&mut self, ctx: &mut HIRContext, ty: Type) -> ValueId {
        let id = ctx.program_builder.new_value_id();
        ctx.program_builder.value_types.insert(id, ty);

        self.value_definitions.insert(id, self.current_block_id);

        id
    }

    pub fn append_block_param(
        &mut self,
        ctx: &mut HIRContext,
        block_id: BasicBlockId,
        ty: Type,
    ) -> ValueId {
        let id = ctx.program_builder.new_value_id();
        ctx.program_builder.value_types.insert(id, ty);

        self.value_definitions.insert(id, block_id);

        let block = self.cfg.blocks.get_mut(&block_id).unwrap_or_else(|| {
            panic!(
                "INTERNAL COMPILER ERROR: Could not append basic block parameter, \
                 BasicBlockId({}) not found",
                block_id.0
            )
        });
        block.params.push(id);
        id
    }

    fn add_predecessor(&mut self, target: BasicBlockId, from: BasicBlockId) {
        self.predecessors.entry(target).or_default().push(from);
    }

    pub fn get_mapped_value(
        &self,
        block: BasicBlockId,
        original: ValueId,
    ) -> Option<ValueId> {
        self.block_value_maps
            .get(&block)
            .and_then(|map| map.get(&original).copied())
    }

    pub fn map_value(&mut self, block: BasicBlockId, original: ValueId, local: ValueId) {
        self.block_value_maps
            .entry(block)
            .or_default()
            .insert(original, local);
    }

    pub fn seal_block(&mut self, ctx: &mut HIRContext, block_id: BasicBlockId) {
        if !self.sealed_blocks.insert(block_id) {
            return;
        }

        if let Some(incomplete) = self.incomplete_params.remove(&block_id) {
            for (param_id, original_value_id) in incomplete {
                self.fill_predecessors(ctx, block_id, original_value_id, param_id);
            }
        }
    }

    /// Returns ValueId which represents original_value_id in the **_target_** basic block
    pub fn use_value_in_block(
        &mut self,
        ctx: &mut HIRContext,
        block_id: BasicBlockId,
        original_value_id: ValueId,
    ) -> ValueId {
        if let Some(def_block) = self.value_definitions.get(&original_value_id) {
            if *def_block == block_id {
                return original_value_id;
            }
        }

        if let Some(local_id) = self.get_mapped_value(block_id, original_value_id) {
            return local_id;
        }

        if !self.sealed_blocks.contains(&block_id) {
            // We don't know all predecessors yet, so we MUST create a placeholder parameter.
            // We will fill in the terminator arguments later when we seal.
            let ty = ctx.program_builder.get_value_id_type(&original_value_id);
            let param_id = self.append_block_param(ctx, block_id, ty);

            self.map_value(block_id, original_value_id, param_id);

            self.incomplete_params
                .entry(block_id)
                .or_default()
                .push((param_id, original_value_id));

            return param_id;
        }

        let ty = ctx.program_builder.get_value_id_type(&original_value_id);
        let param_id = self.append_block_param(ctx, block_id, ty);
        self.map_value(block_id, original_value_id, param_id);
        self.fill_predecessors(ctx, block_id, original_value_id, param_id);

        param_id
    }

    fn fill_predecessors(
        &mut self,
        ctx: &mut HIRContext,
        block_id: BasicBlockId,
        original_value_id: ValueId,
        _param_id: ValueId,
    ) {
        let preds = self
            .predecessors
            .get(&block_id)
            .cloned()
            .unwrap_or_default();

        for pred_id in preds {
            let val_in_pred = self.use_value_in_block(ctx, pred_id, original_value_id);

            self.append_arg_to_terminator(pred_id, block_id, val_in_pred);
        }
    }

    pub fn set_basic_block_terminator(&mut self, terminator: Terminator) {
        match &terminator {
            Terminator::Jump { target, .. } => {
                self.add_predecessor(*target, self.current_block_id);
            }
            Terminator::CondJump {
                true_target,
                false_target,
                ..
            } => {
                self.add_predecessor(*true_target, self.current_block_id);
                self.add_predecessor(*false_target, self.current_block_id);
            }
            _ => {}
        }

        let current_basic_block = self.cfg.blocks.get_mut(&self.current_block_id);
        if let Some(bb) = current_basic_block {
            bb.terminator = Some(terminator);
        } else {
            panic!(
                "INTERNAL COMPILER ERROR: Could not set basic block terminator: basic \
                 block with id: {} doesn't exist.",
                self.current_block_id.0
            );
        }
    }

    fn append_arg_to_terminator(
        &mut self,
        from_block: BasicBlockId,
        to_block: BasicBlockId,
        arg: ValueId,
    ) {
        let block = self
            .cfg
            .blocks
            .get_mut(&from_block)
            .expect("INTERNAL COMPILER ERROR: Block not found");
        let terminator = block
            .terminator
            .as_mut()
            .expect("INTERNAL COMPILER ERROR: Terminator not found");

        match terminator {
            Terminator::Jump { target, args } => {
                if *target == to_block {
                    args.push(Value::Use(arg));
                } else {
                    panic!("INTERNAL COMPILER ERROR: Invalid 'to_block' argument")
                }
            }
            Terminator::CondJump {
                true_target,
                true_args,
                false_target,
                false_args,
                ..
            } => {
                if *true_target == to_block {
                    true_args.push(Value::Use(arg));
                }
                if *false_target == to_block {
                    false_args.push(Value::Use(arg));
                }
                if *true_target != to_block && *false_target != to_block {
                    panic!(
                        "INTERNAL COMPILER ERROR: Invalid 'to_block' argument, didn't \
                         match neither 'true_target' nor 'false_target'"
                    )
                }
            }
            _ => {}
        }
    }

    /// Records a semantic error and returns a new "poison" Value of type Unknown
    /// The caller is responsible for immediately returning the poison Value
    pub fn report_error_and_get_poison(
        &mut self,
        ctx: &mut HIRContext,
        error: SemanticError,
    ) -> ValueId {
        ctx.module_builder.errors.push(error);

        self.alloc_value(ctx, Type::Unknown)
    }

    pub fn get_current_basic_block(&mut self) -> &mut BasicBlock {
        self.cfg
            .blocks
            .get_mut(&self.current_block_id)
            .unwrap_or_else(|| {
                panic!(
                    "INTERNAL COMPILER ERROR: Basic block with id '{}' does not exist.",
                    self.current_block_id.0
                )
            })
    }

    pub fn get_refined_type(
        &mut self,
        ctx: &mut HIRContext,
        block_id: BasicBlockId,
        value_id: ValueId,
    ) -> Type {
        if let Some(ty) = self.refinements.get(&(block_id, value_id)) {
            return ty.clone();
        }

        if !self.sealed_blocks.contains(&block_id) {
            return ctx.program_builder.get_value_id_type(&value_id);
        }

        let block = &self.cfg.blocks[&block_id];
        let param_index = block.params.iter().position(|&p| p == value_id);

        let result_type = if let Some(idx) = param_index {
            let preds = self
                .predecessors
                .get(&block_id)
                .cloned()
                .unwrap_or_default();

            if preds.is_empty() {
                ctx.program_builder.get_value_id_type(&value_id)
            } else {
                let mut incoming_types = Vec::with_capacity(preds.len());
                for pred_id in preds {
                    let arg_value =
                        self.get_terminator_arg_for_param(pred_id, block_id, idx);

                    let ty = match arg_value {
                        Value::Use(id) => self.get_refined_type(ctx, pred_id, id),
                        _ => ctx.program_builder.get_value_type(&arg_value),
                    };

                    incoming_types.push((ty, crate::ast::Span::default()));
                }

                try_unify_types(&incoming_types)
                    .unwrap_or_else(|_| ctx.program_builder.get_value_id_type(&value_id))
            }
        } else {
            let preds = self
                .predecessors
                .get(&block_id)
                .cloned()
                .unwrap_or_default();
            if preds.is_empty() {
                ctx.program_builder.get_value_id_type(&value_id)
            } else {
                let mut incoming_types = Vec::with_capacity(preds.len());
                for pred_id in preds {
                    let ty = self.get_refined_type(ctx, pred_id, value_id);
                    incoming_types.push((ty, crate::ast::Span::default()));
                }
                try_unify_types(&incoming_types)
                    .unwrap_or_else(|_| ctx.program_builder.get_value_id_type(&value_id))
            }
        };

        self.refinements
            .insert((block_id, value_id), result_type.clone());
        result_type
    }

    /// Helper to find which ValueId is passed to a specific block parameter index
    fn get_terminator_arg_for_param(
        &self,
        from_block: BasicBlockId,
        to_block: BasicBlockId,
        param_index: usize,
    ) -> Value {
        let terminator = self.cfg.blocks[&from_block]
            .terminator
            .as_ref()
            .expect("Block must have terminator");

        match terminator {
            Terminator::Jump { target, args } => {
                assert_eq!(target, &to_block);
                args[param_index].clone()
            }
            Terminator::CondJump {
                true_target,
                true_args,
                false_target,
                false_args,
                ..
            } => {
                if true_target == &to_block {
                    true_args[param_index].clone()
                } else if false_target == &to_block {
                    false_args[param_index].clone()
                } else {
                    panic!("Inconsistent CFG: target block not found in CondJump")
                }
            }
            _ => panic!("Terminator type does not support block arguments"),
        }
    }
}
