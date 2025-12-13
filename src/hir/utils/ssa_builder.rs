use crate::{
    ast::{IdentifierNode, Span},
    hir::{
        cfg::{BasicBlock, BasicBlockId, DeclarationId, Terminator, Value, ValueId},
        errors::{SemanticError, SemanticErrorKind},
        types::{checked_declaration::CheckedDeclaration, checked_type::Type},
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn use_basic_block(&mut self, id: BasicBlockId) {
        if let Some(_) = self.cfg.blocks.get(&id) {
            self.current_block_id = id;
        } else {
            panic!(
                "INTERNAL COMPILER ERROR: Could not use basic block with id {} as it doesn't exist",
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

        let block = self.cfg.blocks.get_mut(&block_id).expect(&format!(
            "INTERNAL COMPILER ERROR: Could not append basic block parameter, BasicBlockId({}) not found",
            block_id.0,
        ));
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

        if let Some(incomplete_vars) = self.incomplete_variables.remove(&block_id) {
            for (var, param_id) in incomplete_vars {
                let preds = self
                    .predecessors
                    .get(&block_id)
                    .cloned()
                    .unwrap_or_default();

                // Note: In a loop header, the backedge might provide a narrower type,
                // but the param was created with the Constraint type to be safe.
                for pred in preds {
                    let val = self.read_variable_from_block(ctx, pred, var);
                    let val_in_pred = self.use_value_in_block(ctx, pred, val);
                    self.append_arg_to_terminator(pred, block_id, val_in_pred);
                }
            }
        }
    }

    pub fn write_variable(&mut self, var: DeclarationId, value: ValueId) {
        self.var_to_current_valueid
            .entry(self.current_block_id)
            .or_default()
            .insert(var, value);
    }

    pub fn read_variable(&mut self, ctx: &mut HIRContext, var: DeclarationId) -> ValueId {
        if let Some(map) = self.var_to_current_valueid.get(&self.current_block_id) {
            if let Some(val) = map.get(&var) {
                return *val;
            }
        }

        self.read_variable_recursive(ctx, self.current_block_id, var)
    }

    fn read_variable_from_block(
        &mut self,
        ctx: &mut HIRContext,
        block_id: BasicBlockId,
        var: DeclarationId,
    ) -> ValueId {
        if let Some(map) = self.var_to_current_valueid.get(&block_id) {
            if let Some(val) = map.get(&var) {
                return *val;
            }
        }
        self.read_variable_recursive(ctx, block_id, var)
    }

    fn read_variable_recursive(
        &mut self,
        ctx: &mut HIRContext,
        block_id: BasicBlockId,
        var: DeclarationId,
    ) -> ValueId {
        if !self.sealed_blocks.contains(&block_id) {
            // We don't know predecessors yet, so we have to use the original declared constraint
            let decl = ctx.program_builder.get_declaration(var);
            let constraint_type = match decl {
                CheckedDeclaration::Var(v) => v.constraint.clone(),
                _ => Type::Unknown,
            };

            let param_id = self.append_block_param(ctx, block_id, constraint_type);

            self.incomplete_variables
                .entry(block_id)
                .or_default()
                .push((var, param_id));

            self.var_to_current_valueid
                .entry(block_id)
                .or_default()
                .insert(var, param_id);

            return param_id;
        }

        let preds = self
            .predecessors
            .get(&block_id)
            .cloned()
            .unwrap_or_default();

        if preds.is_empty() {
            let decl = ctx.program_builder.get_declaration(var);
            let identifier = match decl {
                CheckedDeclaration::Var(v) => v.identifier,
                CheckedDeclaration::UninitializedVar { identifier, .. } => *identifier,
                _ => IdentifierNode {
                    name: ctx.program_builder.common_identifiers.id,
                    span: Span::default(),
                }, // Should not happen
            };

            return self.report_error_and_get_poison(
                ctx,
                SemanticError {
                    kind: SemanticErrorKind::UseOfUninitializedVariable(identifier),
                    span: identifier.span,
                },
            );
        }

        if preds.len() == 1 {
            let val = self.read_variable_from_block(ctx, preds[0], var);
            self.write_variable(var, val);
            self.var_to_current_valueid
                .entry(block_id)
                .or_default()
                .insert(var, val);
            return val;
        }

        let mut incoming_values = Vec::new();
        let mut incoming_types = Vec::new();

        for pred in &preds {
            let val = self.read_variable_from_block(ctx, *pred, var);
            incoming_values.push(val);
            incoming_types.push(ctx.program_builder.get_value_type(&Value::Use(val)));
        }

        let type_entries: Vec<(Type, Span)> = incoming_types
            .into_iter()
            .map(|t| (t, Span::default()))
            .collect();

        let narrowed_type = match self.try_unify_types(&type_entries) {
            Ok(ty) => ty,
            Err(_) => {
                // If unification fails (e.g. incompatible types),
                // we fall back to the declared constraint of the variable.
                // This is safe because all assignments were checked against it.
                let decl = ctx.program_builder.get_declaration(var);
                match decl {
                    CheckedDeclaration::Var(v) => v.constraint.clone(),
                    _ => Type::Unknown,
                }
            }
        };

        let param_id = self.append_block_param(ctx, block_id, narrowed_type);

        self.var_to_current_valueid
            .entry(block_id)
            .or_default()
            .insert(var, param_id);

        for (pred, val) in preds.iter().zip(incoming_values.into_iter()) {
            let val_in_pred = self.use_value_in_block(ctx, *pred, val);
            self.append_arg_to_terminator(*pred, block_id, val_in_pred);
        }

        param_id
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
                "INTERNAL COMPILER ERROR: Could not set basic block terminator: basic block with id: {} doesn't exist.",
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
                    panic!("INTERNAL COMPILER ERROR: Invalid 'to_block' argument, didn't match neither 'true_target' nor 'false_target'")
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
        ctx.program_builder.errors.push(error);
        let unknown_result_id = self.alloc_value(ctx, Type::Unknown);
        unknown_result_id
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
}
