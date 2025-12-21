use std::collections::HashSet;

use crate::{
    ast::{IdentifierNode, Span},
    hir::{
        cfg::{
            BinaryOperationKind, ConstantId, Instruction, UnaryOperationKind, Value,
            ValueId,
        },
        errors::{SemanticError, SemanticErrorKind},
        types::checked_type::{StructKind, Type},
        utils::{check_is_equatable::check_is_equatable, numeric::is_signed},
        FunctionBuilder, HIRContext, ModuleBuilder,
    },
};

impl FunctionBuilder {
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
        left_span: Span,
        right: Value,
        right_span: Span,
    ) -> Result<ValueId, SemanticError> {
        let left_type = ctx.program_builder.get_value_type(&left);
        let right_type = ctx.program_builder.get_value_type(&right);
        let combined_span = Span {
            start: left_span.start,
            end: right_span.end,
        };

        let destination_type = match op_kind {
            BinaryOperationKind::Add
            | BinaryOperationKind::Subtract
            | BinaryOperationKind::Multiply
            | BinaryOperationKind::Divide
            | BinaryOperationKind::Modulo => self.check_binary_numeric_operation(
                &left_type,
                left_span,
                &right_type,
                right_span,
            )?,

            BinaryOperationKind::LessThan
            | BinaryOperationKind::LessThanOrEqual
            | BinaryOperationKind::GreaterThan
            | BinaryOperationKind::GreaterThanOrEqual => {
                self.check_binary_numeric_operation(
                    &left_type,
                    left_span,
                    &right_type,
                    right_span,
                )?;

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
