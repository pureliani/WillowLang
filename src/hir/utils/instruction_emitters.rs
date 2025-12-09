use std::collections::HashSet;

use crate::{
    ast::{IdentifierNode, Span},
    hir::{
        cfg::{
            BasicBlock, BasicBlockId, BinaryOperationKind, ConstantId, Instruction,
            Terminator, UnaryOperationKind, Value, ValueId,
        },
        errors::{SemanticError, SemanticErrorKind},
        types::checked_type::{StructKind, Type},
        utils::{check_is_equatable::check_is_equatable, numeric::is_signed},
        FunctionBuilder, HIRContext, ModuleBuilder,
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
    }

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
            // We don't know all predecessors yet, so we MUST create a parameter
            // to be safe. We will fill in the arguments later when we seal.
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
        block_id: BasicBlockId,
        target_block: BasicBlockId,
        arg: ValueId,
    ) {
        let block = self.cfg.blocks.get_mut(&block_id).expect("Block not found");
        let terminator = block.terminator.as_mut().expect("Terminator not found");

        match terminator {
            Terminator::Jump { target, args } => {
                if *target == target_block {
                    args.push(Value::Use(arg));
                }
            }
            Terminator::CondJump {
                true_target,
                true_args,
                false_target,
                false_args,
                ..
            } => {
                if *true_target == target_block {
                    true_args.push(Value::Use(arg));
                }
                if *false_target == target_block {
                    false_args.push(Value::Use(arg));
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

    fn push_instruction(&mut self, instruction: Instruction) {
        let current_block = self.get_current_basic_block();

        if current_block.terminator.is_some() {
            panic!(
                "INTERNAL COMPILER ERROR: Attempted to add instruction to a basic block (ID: {}) that has already been terminated",
                current_block.id.0
            );
        }

        current_block.instructions.push(instruction);
    }

    /// Returns ValueId which holds pointer: Type::Pointer(Box<Type>)
    pub fn emit_stack_alloc(
        &mut self,
        ctx: &mut HIRContext,
        ty: Type,
        count: usize,
    ) -> ValueId {
        let destination = self.alloc_value(ctx, Type::Pointer(Box::new(ty)));
        self.push_instruction(Instruction::StackAlloc { destination, count });

        destination
    }

    /// Returns ValueId which holds pointer: Type::Pointer(Box<Type>)
    pub fn emit_heap_alloc(
        &mut self,
        ctx: &mut HIRContext,
        ty: Type,
        count: Value,
    ) -> Result<ValueId, SemanticError> {
        let count_type = ctx.program_builder.get_value_type(&count);
        let expected_count_type = Type::USize;
        if !self.check_is_assignable(&count_type, &expected_count_type) {
            return Err(SemanticError {
                span: Span::default(), // TODO: Fix span propagation
                kind: SemanticErrorKind::TypeMismatch {
                    expected: expected_count_type,
                    received: count_type,
                },
            });
        }

        let destination = self.alloc_value(ctx, Type::Pointer(Box::new(ty)));
        self.push_instruction(Instruction::HeapAlloc { destination, count });

        Ok(destination)
    }

    pub fn emit_store(&mut self, ctx: &mut HIRContext, ptr: ValueId, value: Value) {
        let value_type = ctx.program_builder.get_value_type(&value);
        let destination_ptr_type = ctx.program_builder.get_value_id_type(&ptr);

        if let Type::Pointer(target_type) = destination_ptr_type {
            if !self.check_is_assignable(&value_type, &target_type) {
                ctx.program_builder.errors.push(SemanticError {
                    span: Span::default(), // TODO: Fix span
                    kind: SemanticErrorKind::TypeMismatch {
                        expected: *target_type,
                        received: value_type,
                    },
                });
                return;
            }
        } else {
            panic!("INTERNAL COMPILER ERROR: Expected destination_ptr_id to be of Pointer<T> type");
        }

        self.push_instruction(Instruction::Store { ptr, value });
    }

    pub fn emit_load(&mut self, ctx: &mut HIRContext, ptr: ValueId) -> ValueId {
        let ptr_type = ctx.program_builder.get_value_id_type(&ptr);

        let destination_type = if let Type::Pointer(target_type) = ptr_type {
            *target_type
        } else {
            panic!(
                "INTERNAL COMPILER ERROR: Expected source_ptr to be of Pointer<T> type"
            );
        };

        let destination = self.alloc_value(ctx, destination_type);
        self.push_instruction(Instruction::Load { destination, ptr });

        destination
    }

    pub fn emit_load_constant(
        &mut self,
        ctx: &mut HIRContext,
        constant_id: ConstantId,
    ) -> ValueId {
        let ptr_type = Type::Pointer(Box::new(Type::U8));

        let destination = self.alloc_value(ctx, ptr_type);

        self.push_instruction(Instruction::LoadConstant {
            destination,
            constant_id,
        });

        destination
    }

    pub fn emit_get_field_ptr(
        &mut self,
        ctx: &mut HIRContext,
        base_ptr: ValueId,
        field: IdentifierNode,
    ) -> Result<ValueId, SemanticError> {
        let base_ptr_type = ctx.program_builder.get_value_id_type(&base_ptr);

        let s = if let Type::Pointer(ptr_to) = base_ptr_type {
            if let Type::Struct(s) = *ptr_to {
                s
            } else {
                return Err(SemanticError {
                    kind: SemanticErrorKind::CannotAccess(ptr_to.as_ref().clone()),
                    span: field.span,
                });
            }
        } else {
            panic!("INTERNAL COMPILER ERROR: emit_get_field_ptr called on a non-pointer type.");
        };

        if let Some((field_index, ty)) = s.get_field(&ctx.program_builder, field.name) {
            let destination = self.alloc_value(ctx, Type::Pointer(Box::new(ty.clone())));

            self.push_instruction(Instruction::GetFieldPtr {
                destination,
                base_ptr,
                field_index,
            });

            Ok(destination)
        } else {
            Err(SemanticError {
                kind: SemanticErrorKind::AccessToUndefinedField(field),
                span: field.span,
            })
        }
    }

    pub fn emit_get_element_ptr(
        &mut self,
        ctx: &mut HIRContext,
        base_ptr: ValueId,
        index: Value,
    ) -> Result<ValueId, SemanticError> {
        let index_type = ctx.program_builder.get_value_type(&index);
        if !self.check_is_assignable(&index_type, &Type::USize) {
            return Err(SemanticError {
                span: Span::default(),
                kind: SemanticErrorKind::TypeMismatch {
                    expected: Type::USize,
                    received: index_type,
                },
            });
        }

        let base_ptr_type = ctx.program_builder.get_value_id_type(&base_ptr);
        let element_type = if let Type::Pointer(inner) = base_ptr_type {
            *inner
        } else {
            panic!("INTERNAL COMPILER ERROR: emit_get_element_ptr expects a pointer");
        };

        let destination = self.alloc_value(ctx, Type::Pointer(Box::new(element_type)));
        self.push_instruction(Instruction::GetElementPtr {
            destination,
            base_ptr,
            index,
        });

        Ok(destination)
    }

    pub fn emit_unary_op(
        &mut self,
        ctx: &mut HIRContext,
        op_kind: UnaryOperationKind,
        value: Value,
    ) -> Result<ValueId, SemanticError> {
        let value_type = ctx.program_builder.get_value_type(&value);
        let span = Span::default(); // TODO: Fix span

        match op_kind {
            UnaryOperationKind::Neg => {
                if !is_signed(&value_type) {
                    let expected = HashSet::from([
                        Type::I8,
                        Type::I16,
                        Type::I32,
                        Type::I64,
                        Type::ISize,
                        Type::F32,
                        Type::F64,
                    ]);

                    return Err(SemanticError {
                        kind: SemanticErrorKind::TypeMismatchExpectedOneOf {
                            expected,
                            received: value_type.clone(),
                        },
                        span,
                    });
                }
            }
            UnaryOperationKind::Not => {
                let bool_type = Type::Bool;

                if !self.check_is_assignable(&value_type, &bool_type) {
                    return Err(SemanticError {
                        kind: SemanticErrorKind::TypeMismatch {
                            expected: bool_type.clone(),
                            received: value_type,
                        },
                        span,
                    });
                }
            }
        };

        let destination = self.alloc_value(ctx, value_type);
        self.push_instruction(Instruction::UnaryOp {
            op_kind,
            destination,
            operand: value,
        });

        Ok(destination)
    }

    pub fn emit_binary_op(
        &mut self,
        ctx: &mut HIRContext,
        op_kind: BinaryOperationKind,
        left: Value,
        right: Value,
    ) -> Result<ValueId, SemanticError> {
        let left_type = ctx.program_builder.get_value_type(&left);
        let right_type = ctx.program_builder.get_value_type(&right);
        let combined_span = Span::default(); // TODO: Fix span

        let destination_type = match op_kind {
            BinaryOperationKind::Add
            | BinaryOperationKind::Subtract
            | BinaryOperationKind::Multiply
            | BinaryOperationKind::Divide
            | BinaryOperationKind::Modulo => {
                self.check_binary_numeric_operation(&left_type, &right_type)?
            }

            BinaryOperationKind::LessThan
            | BinaryOperationKind::LessThanOrEqual
            | BinaryOperationKind::GreaterThan
            | BinaryOperationKind::GreaterThanOrEqual => {
                self.check_binary_numeric_operation(&left_type, &right_type)?;

                Type::Bool
            }

            BinaryOperationKind::Equal | BinaryOperationKind::NotEqual => {
                if !check_is_equatable(&left_type, &right_type) {
                    return Err(SemanticError {
                        span: combined_span,
                        kind: SemanticErrorKind::CannotCompareType {
                            of: left_type,
                            to: right_type,
                        },
                    });
                }

                Type::Bool
            }
        };

        let destination = self.alloc_value(ctx, destination_type);
        self.push_instruction(Instruction::BinaryOp {
            op_kind,
            destination,
            left,
            right,
        });

        Ok(destination)
    }

    pub fn emit_function_call(
        &mut self,
        ctx: &mut HIRContext,
        value: Value,
        args: Vec<Value>,
        call_span: Span,
    ) -> Result<Option<ValueId>, SemanticError> {
        let value_type = ctx.program_builder.get_value_type(&value);

        let (params, return_type) = match &value_type {
            Type::Fn(fn_type) => (fn_type.params.clone(), fn_type.return_type.clone()),
            Type::Struct(s) if matches!(s, StructKind::ClosureObject(_)) => {
                todo!();
            }
            _ => {
                return Err(SemanticError {
                    kind: SemanticErrorKind::CannotCall(value_type),
                    span: call_span,
                });
            }
        };

        if args.len() != params.len() {
            return Err(SemanticError {
                kind: SemanticErrorKind::FnArgumentCountMismatch {
                    expected: params.len(),
                    received: args.len(),
                },
                span: call_span,
            });
        }

        for (arg_value, param_decl) in args.iter().zip(params.iter()) {
            let arg_type = ctx.program_builder.get_value_type(arg_value);
            let param_type = &param_decl.ty;

            if !self.check_is_assignable(&arg_type, param_type) {
                return Err(SemanticError {
                    span: Span::default(), // TODO: Fix span
                    kind: SemanticErrorKind::TypeMismatch {
                        expected: param_type.clone(),
                        received: arg_type,
                    },
                });
            }
        }

        let destination_id = if *return_type != Type::Void {
            Some(self.alloc_value(ctx, *return_type))
        } else {
            None
        };

        self.push_instruction(Instruction::FunctionCall {
            destination: destination_id,
            function_rvalue: value,
            args,
        });

        Ok(destination_id)
    }

    pub fn emit_type_cast(
        &mut self,
        ctx: &mut HIRContext,
        value: Value,
        target_type: Type,
    ) -> ValueId {
        let value_type = ctx.program_builder.get_value_type(&value);

        if !self.check_is_casting_allowed(&value_type, &target_type) {
            return self.report_error_and_get_poison(
                ctx,
                SemanticError {
                    span: Span::default(), // TODO: Fix span
                    kind: SemanticErrorKind::CannotCastType {
                        source_type: value_type.clone(),
                        target_type: target_type.clone(),
                    },
                },
            );
        }

        let destination = self.alloc_value(ctx, target_type.clone());
        self.push_instruction(Instruction::TypeCast {
            destination,
            operand: value,
            target_type,
        });

        destination
    }

    pub fn emit_nop(&mut self, _module_builder: &mut ModuleBuilder) {
        self.push_instruction(Instruction::Nop);
    }
}
