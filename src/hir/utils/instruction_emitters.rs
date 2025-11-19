use std::collections::HashSet;

use crate::{
    ast::{IdentifierNode, Span},
    hir::{
        cfg::{
            BasicBlock, BasicBlockId, BinaryOperationKind, Instruction, IntrinsicField,
            IntrinsicFunction, Terminator, UnaryOperationKind, Value, ValueId,
        },
        errors::{SemanticError, SemanticErrorKind},
        types::checked_type::{CheckedStruct, StructKind, Type},
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

    pub fn set_basic_block_terminator(&mut self, terminator: Terminator) {
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

    /// Records a semantic error and returns a new "poison" Value of type Unknown.
    /// The caller is responsible for immediately returning the poison Value.
    pub fn report_error_and_get_poison(
        &mut self,
        ctx: &mut HIRContext,
        error: SemanticError,
    ) -> ValueId {
        ctx.program_builder.errors.push(error);
        let unknown_result_id = ctx.program_builder.new_value_id();
        ctx.program_builder
            .value_types
            .insert(unknown_result_id, Type::Unknown);
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
        let destination = ctx.program_builder.new_value_id();

        ctx.program_builder
            .value_types
            .insert(destination, Type::Pointer(Box::new(ty)));

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
            // Note: We need a span for the error. Since Type no longer has span,
            // we rely on the caller or context. Here we assume count_type came from an expression
            // but we don't have the expression span.
            // Ideally, `get_value_type` or `Value` should carry span info, or we pass it in.
            // For now, we use a default span or fix this upstream.
            return Err(SemanticError {
                span: Span::default(), // TODO: Fix span propagation
                kind: SemanticErrorKind::TypeMismatch {
                    expected: expected_count_type,
                    received: count_type,
                },
            });
        }

        let destination = ctx.program_builder.new_value_id();
        let allocation_site_id = ctx.program_builder.new_allocation_id();

        ctx.program_builder
            .value_types
            .insert(destination, Type::Pointer(Box::new(ty)));

        self.push_instruction(Instruction::HeapAlloc {
            destination,
            allocation_site_id,
            count,
        });

        Ok(destination)
    }

    pub fn emit_store(
        &mut self,
        ctx: &mut HIRContext,
        destination_ptr: ValueId,
        value: Value,
    ) {
        let value_type = ctx.program_builder.get_value_type(&value);
        let destination_ptr_type =
            ctx.program_builder.get_value_id_type(&destination_ptr);

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

        self.push_instruction(Instruction::Store {
            destination_ptr,
            source_val: value,
        });
    }

    pub fn emit_load(&mut self, ctx: &mut HIRContext, source_ptr: ValueId) -> ValueId {
        let ptr_type = ctx.program_builder.get_value_id_type(&source_ptr);

        let destination_type = if let Type::Pointer(target_type) = ptr_type {
            *target_type
        } else {
            panic!(
                "INTERNAL COMPILER ERROR: Expected source_ptr to be of Pointer<T> type"
            );
        };

        let destination = ctx.program_builder.new_value_id();

        ctx.program_builder
            .value_types
            .insert(destination, destination_type);

        self.push_instruction(Instruction::Load {
            destination,
            source_ptr,
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

        if let Some((field_index, ty)) = s.get_field(field.name) {
            let destination = ctx.program_builder.new_value_id();

            ctx.program_builder
                .value_types
                .insert(destination, Type::Pointer(Box::new(ty.clone())));

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

    pub fn emit_intrinsic_function_call(
        &mut self,
        ctx: &mut HIRContext,
        function_kind: IntrinsicFunction,
    ) -> Result<Option<ValueId>, SemanticError> {
        match function_kind {
            IntrinsicFunction::ListSet {
                list_base_ptr,
                index,
                item,
            } => {
                let index_type = ctx.program_builder.get_value_type(&index);

                if !self.check_is_assignable(&index_type, &Type::USize) {
                    return Err(SemanticError {
                        span: Span::default(), // TODO: Fix span
                        kind: SemanticErrorKind::TypeMismatch {
                            expected: Type::USize,
                            received: index_type,
                        },
                    });
                }

                let item_type = ctx.program_builder.get_value_type(&item);
                let list_type = ctx.program_builder.get_value_id_type(&list_base_ptr);

                let Type::Pointer(ptr_to) = &list_type else {
                    panic!("INTERNAL COMPILER ERROR: Called ListSet intrinsic function on a non-pointer type");
                };

                let Type::Struct(checked_struct) = ptr_to.as_ref() else {
                    panic!("INTERNAL COMPILER ERROR: Called ListSet intrinsic function on a pointer to a non-struct type");
                };

                if !matches!(checked_struct.kind(), StructKind::List) {
                    panic!("INTERNAL COMPILER ERROR: Called ListSet intrinsic function on a pointer to a non-list struct");
                }

                let ptr_field_name = ctx.program_builder.common_identifiers.ptr;
                let (_, ptr_field_type) = checked_struct
                    .get_field(ptr_field_name)
                    .expect("INTERNAL COMPILER ERROR: List struct missing 'ptr' field");

                let Type::Pointer(element_type) = ptr_field_type else {
                    panic!("INTERNAL COMPILER ERROR: List 'ptr' field is not a pointer");
                };

                if !self.check_is_assignable(&item_type, element_type) {
                    return Err(SemanticError {
                        span: Span::default(),
                        kind: SemanticErrorKind::TypeMismatch {
                            expected: *element_type.clone(),
                            received: item_type,
                        },
                    });
                }

                self.push_instruction(Instruction::IntrinsicFunctionCall(
                    IntrinsicFunction::ListSet {
                        list_base_ptr,
                        index,
                        item,
                    },
                ));

                Ok(None)
            }
            IntrinsicFunction::ListGet {
                list_base_ptr,
                index,
                destination,
            } => todo!(),
        }
    }
    pub fn emit_intrinsic_field_access(
        &mut self,
        ctx: &mut HIRContext,
        function_kind: IntrinsicField,
    ) {
        todo!()
    }

    pub fn emit_unary_op(
        &mut self,
        ctx: &mut HIRContext,
        op_kind: UnaryOperationKind,
        value: Value,
    ) -> Result<ValueId, SemanticError> {
        let value_type = ctx.program_builder.get_value_type(&value);
        let span = Span::default(); // TODO: Fix span

        let destination = match op_kind {
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

                ctx.program_builder.new_value_id()
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

                ctx.program_builder.new_value_id()
            }
        };

        ctx.program_builder
            .value_types
            .insert(destination, value_type);
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

        let destination = ctx.program_builder.new_value_id();
        ctx.program_builder
            .value_types
            .insert(destination, destination_type);
        self.push_instruction(Instruction::BinaryOp {
            op_kind,
            destination,
            left,
            right,
        });

        Ok(destination)
    }

    pub fn emit_phi(
        &mut self,
        ctx: &mut HIRContext,
        sources: Vec<(BasicBlockId, Value)>,
    ) -> Result<ValueId, SemanticError> {
        if sources.is_empty() {
            panic!("INTERNAL COMPILER ERROR: emit_phi called with no sources.");
        }

        let source_types: Vec<Type> = sources
            .iter()
            .map(|(_, val)| ctx.program_builder.get_value_type(val))
            .collect();

        let first_type = source_types[0].clone();
        let mut are_all_tags = true;
        let mut union_tags = Vec::new();

        for ty in &source_types {
            if let Type::Struct(s) = ty {
                if matches!(s.kind(), StructKind::Tag) {
                    if !union_tags.contains(ty) {
                        union_tags.push(ty.clone());
                    }
                } else {
                    are_all_tags = false;
                    break;
                }
            } else {
                are_all_tags = false;
                break;
            }
        }

        let result_type = if are_all_tags {
            // Create a Union from the collected tags
            Type::Struct(CheckedStruct::union(&ctx.program_builder, &union_tags))
        } else {
            for other_type in source_types.iter().skip(1) {
                if !self.check_is_assignable(other_type, &first_type) {
                    return Err(SemanticError {
                        span: Span::default(), // TODO: Fix span
                        kind: SemanticErrorKind::IncompatibleBranchTypes {
                            first: first_type,
                            second: other_type.clone(),
                        },
                    });
                }
            }
            first_type
        };

        let destination = ctx.program_builder.new_value_id();
        ctx.program_builder
            .value_types
            .insert(destination, result_type);

        self.push_instruction(Instruction::Phi {
            destination,
            sources,
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
            Type::Struct(s) if matches!(s.kind(), StructKind::Closure) => {
                // A closure wrapper is { fn_ptr, env_ptr }.
                // We need to find the actual function signature.
                // However, the Type::Struct(Closure) itself doesn't carry the signature info
                // in its fields (it just has void* pointers).
                //
                // This implies that `value_type` for a closure variable MUST carry the signature info
                // somehow.
                //
                // In your previous design, TypeKind::Closure had the signature.
                // Now, Type::Struct(Closure) is just the memory layout.
                //
                // SOLUTION:
                // You likely need a `Type::Closure(Box<CheckedFnType>)` or similar high-level type
                // that *lowers* to `Type::Struct(Closure)` during codegen, OR
                // you need to store the signature in `StructKind::Closure(Signature)`.
                //
                // Given your current `StructKind::Closure` has no data, we have a problem here.
                // The type system has lost the signature information.

                // TEMPORARY FIX ASSUMPTION:
                // You might want to change StructKind::Closure to StructKind::Closure(Box<CheckedFnType>)
                // so the type system knows what the closure expects.

                todo!("Implement closure signature tracking in Type system");
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
            let dest_id = ctx.program_builder.new_value_id();
            ctx.program_builder
                .value_types
                .insert(dest_id, *return_type);
            Some(dest_id)
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

        let destination = ctx.program_builder.new_value_id();
        ctx.program_builder
            .value_types
            .insert(destination, target_type.clone());

        self.push_instruction(Instruction::TypeCast {
            destination,
            operand: value,
            target_type,
        });

        destination
    }

    pub fn emit_nop(&mut self, module_builder: &mut ModuleBuilder) {
        todo!()
    }
}
