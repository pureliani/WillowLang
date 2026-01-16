use std::collections::HashSet;

use crate::{
    ast::{IdentifierNode, Span},
    hir::{
        cfg::{
            BinaryOperationKind, ConstantId, Instruction, UnaryOperationKind, Value,
            ValueId,
        },
        errors::{SemanticError, SemanticErrorKind},
        types::{checked_declaration::CheckedDeclaration, checked_type::Type},
        utils::{
            check_is_assignable::check_is_assignable,
            check_is_equatable::check_is_equatable, numeric::is_signed,
        },
        FunctionBuilder, HIRContext, ModuleBuilder,
    },
};

impl FunctionBuilder {
    fn push_instruction(&mut self, instruction: Instruction) {
        let current_block = self.get_current_basic_block();

        if current_block.terminator.is_some() {
            panic!(
                "INTERNAL COMPILER ERROR: Attempted to add instruction to a basic block \
                 (ID: {}) that has already been terminated",
                current_block.id.0
            );
        }

        current_block.instructions.push(instruction);
    }

    /// Returns ValueId which holds pointer: Type::Pointer { kind: PointerKind::Raw, to: Box<Type> }
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

    /// Returns ValueId which holds pointer: Type::Pointer { kind: PointerKind::Raw, to: Box<Type> }
    pub fn emit_heap_alloc(
        &mut self,
        ctx: &mut HIRContext,
        ty: Type,
        count: Value,
    ) -> Result<ValueId, SemanticError> {
        let count_type = ctx.program_builder.get_value_type(&count);
        let expected_count_type = Type::USize;
        if !check_is_assignable(&count_type, &expected_count_type) {
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

        let target_type = match destination_ptr_type {
            Type::Pointer(to) => to,
            _ => panic!(
                "INTERNAL COMPILER ERROR: Expected destination_ptr_id to be of Pointer \
                 type"
            ),
        };

        if !check_is_assignable(&value_type, &target_type) {
            ctx.module_builder.errors.push(SemanticError {
                span: Span::default(), // TODO: Fix span
                kind: SemanticErrorKind::TypeMismatch {
                    expected: *target_type,
                    received: value_type,
                },
            });
            return;
        }

        self.push_instruction(Instruction::Store { ptr, value });
    }

    pub fn emit_load(&mut self, ctx: &mut HIRContext, ptr: ValueId) -> ValueId {
        let ptr_type = ctx.program_builder.get_value_id_type(&ptr);

        let destination_type = match ptr_type {
            Type::Pointer(to) => *to,
            _ => panic!(
                "INTERNAL COMPILER ERROR: Expected source_ptr to be of Pointer type"
            ),
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

        let s = match base_ptr_type {
            Type::Pointer(to) => {
                if let Type::Struct(s) = *to {
                    s
                } else {
                    return Err(SemanticError {
                        kind: SemanticErrorKind::CannotAccess(to.as_ref().clone()),
                        span: field.span,
                    });
                }
            }
            _ => panic!(
                "INTERNAL COMPILER ERROR: emit_get_field_ptr called on a non-pointer \
                 type."
            ),
        };

        if let Some((field_index, ty)) = s.get_field(ctx.program_builder, field.name) {
            let result_type = Type::Pointer(Box::new(ty.clone()));

            let destination = self.alloc_value(ctx, result_type);

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
        if !check_is_assignable(&index_type, &Type::USize) {
            return Err(SemanticError {
                span: Span::default(),
                kind: SemanticErrorKind::TypeMismatch {
                    expected: Type::USize,
                    received: index_type,
                },
            });
        }

        let base_ptr_type = ctx.program_builder.get_value_id_type(&base_ptr);
        let element_type = match base_ptr_type {
            Type::Pointer(to) => *to,
            _ => {
                panic!("INTERNAL COMPILER ERROR: emit_get_element_ptr expects a pointer")
            }
        };

        let result_type = Type::Pointer(Box::new(element_type));

        let destination = self.alloc_value(ctx, result_type);
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

                if !check_is_assignable(&value_type, &bool_type) {
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
        function_value: Value,
        args: Vec<Value>,
        call_span: Span,
    ) -> Result<Option<ValueId>, SemanticError> {
        let (fn_ptr_val, params, return_type) = match function_value {
            Value::Function(decl_id) => {
                let decl = ctx.program_builder.get_declaration(decl_id);
                match decl {
                    CheckedDeclaration::Function(f) => (
                        Value::Function(decl_id),
                        f.params.clone(),
                        f.return_type.clone(),
                    ),
                    _ => panic!(
                        "INTERNAL COMPILER ERROR: Value::Function(DeclarationId) \
                         contained non-function declaration id"
                    ),
                }
            }
            Value::Use(val_id) => {
                let ty = ctx.program_builder.get_value_id_type(&val_id);
                match ty {
                    Type::Fn(fn_type) => (
                        Value::Use(val_id),
                        fn_type.params.clone(),
                        *fn_type.return_type.clone(),
                    ),
                    _ => {
                        return Err(SemanticError {
                            kind: SemanticErrorKind::CannotCall(ty),
                            span: call_span,
                        })
                    }
                }
            }
            _ => {
                return Err(SemanticError {
                    kind: SemanticErrorKind::CannotCall(Type::Unknown),
                    span: call_span,
                })
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

        for (arg, param) in args.iter().zip(params.iter()) {
            let arg_ty = ctx.program_builder.get_value_type(arg);
            if !check_is_assignable(&param.ty, &arg_ty) {
                return Err(SemanticError {
                    kind: SemanticErrorKind::TypeMismatch {
                        expected: param.ty.clone(),
                        received: arg_ty,
                    },
                    span: call_span,
                });
            }
        }

        let destination_id = if return_type != Type::Void {
            Some(self.alloc_value(ctx, return_type))
        } else {
            None
        };

        self.push_instruction(Instruction::FunctionCall {
            destination: destination_id,
            function_rvalue: fn_ptr_val,
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
