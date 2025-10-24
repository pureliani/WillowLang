use std::collections::HashSet;

use crate::{
    ast::{IdentifierNode, Span},
    hir::{
        cfg::{
            BasicBlock, BasicBlockId, BinaryOperationKind, Instruction, IntrinsicField,
            IntrinsicFunction, Terminator, UnaryOperationKind, Value, ValueId,
        },
        errors::{SemanticError, SemanticErrorKind},
        types::checked_type::{Type, TypeKind},
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
        let error_span = error.span;
        ctx.program_builder.errors.push(error);
        let unknown_result_id = ctx.program_builder.new_value_id();
        ctx.program_builder.value_types.insert(
            unknown_result_id,
            Type {
                kind: TypeKind::Unknown,
                span: error_span,
            },
        );
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

    /// Returns ValueId which holds pointer: TypeKind::Pointer(Box<Type>)
    pub fn emit_stack_alloc(
        &mut self,
        ctx: &mut HIRContext,
        ty: Type,
        count: usize,
    ) -> ValueId {
        let destination = ctx.program_builder.new_value_id();

        ctx.program_builder.value_types.insert(
            destination,
            Type {
                span: ty.span,
                kind: TypeKind::Pointer(Box::new(ty)),
            },
        );

        self.push_instruction(Instruction::StackAlloc { destination, count });

        destination
    }

    /// Returns ValueId which holds pointer: TypeKind::Pointer(Box<Type>)
    pub fn emit_heap_alloc(
        &mut self,
        ctx: &mut HIRContext,
        ty: Type,
        count: Value,
    ) -> Result<ValueId, SemanticError> {
        let count_type = ctx.program_builder.get_value_type(&count);
        let expected_count_type = Type {
            kind: TypeKind::USize,
            span: count_type.span,
        };
        if !self.check_is_assignable(&count_type, &expected_count_type) {
            return Err(SemanticError {
                span: count_type.span,
                kind: SemanticErrorKind::TypeMismatch {
                    expected: expected_count_type,
                    received: count_type,
                },
            });
        }

        let destination = ctx.program_builder.new_value_id();
        let allocation_site_id = ctx.program_builder.new_allocation_id();

        ctx.program_builder.value_types.insert(
            destination,
            Type {
                span: ty.span,
                kind: TypeKind::Pointer(Box::new(ty)),
            },
        );

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

        if let TypeKind::Pointer(target_type) = destination_ptr_type.kind {
            if !self.check_is_assignable(&value_type, &target_type) {
                ctx.program_builder.errors.push(SemanticError {
                    span: value_type.span,
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

        let destination_type = if let TypeKind::Pointer(target_type) = ptr_type.kind {
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

        let struct_fields = if let TypeKind::Pointer(ptr_to) = &base_ptr_type.kind {
            if let TypeKind::Struct(s) = &ptr_to.kind {
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

        if let Some((field_index, target_field)) = struct_fields
            .iter()
            .enumerate()
            .find(|(_, p)| p.identifier.name == field.name)
        {
            let destination = ctx.program_builder.new_value_id();

            ctx.program_builder.value_types.insert(
                destination,
                Type {
                    kind: TypeKind::Pointer(Box::new(target_field.ty.clone())),
                    span: field.span,
                },
            );

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
                let expected_index_type = &Type {
                    kind: TypeKind::USize,
                    span: index_type.span,
                };
                if !self.check_is_assignable(&index_type, expected_index_type) {
                    return Err(SemanticError {
                        span: index_type.span,
                        kind: SemanticErrorKind::TypeMismatch {
                            expected: expected_index_type.clone(),
                            received: index_type,
                        },
                    });
                }

                let item_type = ctx.program_builder.get_value_type(&item);
                let list_type = ctx.program_builder.get_value_id_type(&list_base_ptr);

                if let TypeKind::Pointer(ptr_to) = &list_type.kind {
                    if let TypeKind::List(expected_list_item_type) = &ptr_to.kind {
                        if !self.check_is_assignable(&item_type, expected_list_item_type)
                        {
                            return Err(SemanticError {
                                span: item_type.span,
                                kind: SemanticErrorKind::TypeMismatch {
                                    expected: *expected_list_item_type.clone(),
                                    received: item_type,
                                },
                            });
                        }
                    } else {
                        panic!("INTERNAL COMPILER ERROR: Called ListSet intrinsic function on a pointer to a non-list type");
                    }
                } else {
                    panic!("INTERNAL COMPILER ERROR: Called ListSet intrinsic function on a non-pointer type");
                }

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
        let span = value_type.span;

        let destination = match op_kind {
            UnaryOperationKind::Neg => {
                if !is_signed(&value_type.kind) {
                    let expected = HashSet::from([
                        Type {
                            kind: TypeKind::I8,
                            span,
                        },
                        Type {
                            kind: TypeKind::I16,
                            span,
                        },
                        Type {
                            kind: TypeKind::I32,
                            span,
                        },
                        Type {
                            kind: TypeKind::I64,
                            span,
                        },
                        Type {
                            kind: TypeKind::ISize,
                            span,
                        },
                        Type {
                            kind: TypeKind::F32,
                            span,
                        },
                        Type {
                            kind: TypeKind::F64,
                            span,
                        },
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
                let bool_type = Type {
                    kind: TypeKind::Bool,
                    span,
                };

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
        let combined_span = Span {
            start: left_type.span.start,
            end: right_type.span.end,
        };

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

                Type {
                    kind: TypeKind::Bool,
                    span: combined_span,
                }
            }

            BinaryOperationKind::Equal | BinaryOperationKind::NotEqual => {
                if !check_is_equatable(&left_type.kind, &right_type.kind) {
                    return Err(SemanticError {
                        span: Span {
                            start: left_type.span.start,
                            end: right_type.span.end,
                        },
                        kind: SemanticErrorKind::CannotCompareType {
                            of: left_type,
                            to: right_type,
                        },
                    });
                }

                Type {
                    kind: TypeKind::Bool,
                    span: combined_span,
                }
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

        let first_type = ctx.program_builder.get_value_type(&sources[0].1);

        for (_, other_value) in sources.iter().skip(1) {
            let other_type = ctx.program_builder.get_value_type(other_value);
            if !self.check_is_assignable(&other_type, &first_type) {
                return Err(SemanticError {
                    span: other_type.span,
                    kind: SemanticErrorKind::IncompatibleBranchTypes {
                        first: first_type,
                        second: other_type,
                    },
                });
            }
        }

        let destination = ctx.program_builder.new_value_id();
        ctx.program_builder
            .value_types
            .insert(destination, first_type);

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

        let fn_type_decl = if let TypeKind::FnType(decl) = value_type.kind {
            decl
        } else {
            return Err(SemanticError {
                kind: SemanticErrorKind::CannotCall(value_type),
                span: call_span,
            });
        };

        if args.len() != fn_type_decl.params.len() {
            return Err(SemanticError {
                kind: SemanticErrorKind::FnArgumentCountMismatch {
                    expected: fn_type_decl.params.len(),
                    received: args.len(),
                },
                span: call_span,
            });
        }

        for (arg_value, param_decl) in args.iter().zip(fn_type_decl.params.iter()) {
            let arg_type = ctx.program_builder.get_value_type(arg_value);
            let param_type = &param_decl.ty;

            if !self.check_is_assignable(&arg_type, param_type) {
                return Err(SemanticError {
                    span: arg_type.span,
                    kind: SemanticErrorKind::TypeMismatch {
                        expected: param_type.clone(),
                        received: arg_type,
                    },
                });
            }
        }

        let destination_id = if fn_type_decl.return_type.kind != TypeKind::Void {
            let dest_id = ctx.program_builder.new_value_id();
            ctx.program_builder
                .value_types
                .insert(dest_id, *fn_type_decl.return_type);
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
                    span: value_type.span,
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
