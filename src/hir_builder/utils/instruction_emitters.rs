use std::collections::HashSet;

use crate::{
    ast::IdentifierNode,
    cfg::{BasicBlock, Instruction, UnaryOperationKind, Value, ValueId},
    hir_builder::{
        errors::{SemanticError, SemanticErrorKind},
        types::checked_type::{Type, TypeKind},
        utils::is_signed::is_signed,
        FunctionBuilder, HIRContext, ModuleBuilder,
    },
};

impl FunctionBuilder {
    /// Records a semantic error and returns a new "poison" Value of type Unknown.
    /// The caller is responsible for immediately returning the poison Value.
    pub fn report_error_and_get_poison(&mut self, ctx: &mut HIRContext, error: SemanticError) -> ValueId {
        let error_span = error.span;
        ctx.module_builder.errors.push(error);
        let unknown_result_id = self.new_value_id();
        self.cfg.value_types.insert(
            unknown_result_id,
            Type {
                kind: TypeKind::Unknown,
                span: error_span,
            },
        );
        unknown_result_id
    }

    pub fn get_current_basic_block(&mut self) -> &mut BasicBlock {
        self.cfg.blocks.get_mut(&self.current_block_id).unwrap_or_else(|| {
            panic!(
                "INTERNAL COMPILER ERROR: Basic block with id '{}' does not exist.",
                self.current_block_id.0
            )
        })
    }

    /// Returns ValueId which holds pointer: TypeKind::Pointer(Box<Type>)
    pub fn emit_alloc(&mut self, ty: Type) -> ValueId {
        let destination = self.new_value_id();

        self.cfg.value_types.insert(
            destination,
            Type {
                span: ty.span,
                kind: TypeKind::Pointer(Box::new(ty)),
            },
        );

        self.get_current_basic_block()
            .instructions
            .push(Instruction::Alloc { destination });

        destination
    }

    /// Returns ValueId which holds pointer: TypeKind::Pointer(Box<Type>)
    pub fn emit_new(&mut self, ctx: &mut HIRContext, ty: Type) -> ValueId {
        let destination = self.new_value_id();
        let allocation_site_id = ctx.program_builder.new_allocation_id();

        self.cfg.value_types.insert(
            destination,
            Type {
                span: ty.span,
                kind: TypeKind::Pointer(Box::new(ty)),
            },
        );

        self.get_current_basic_block().instructions.push(Instruction::New {
            destination,
            allocation_site_id,
        });

        destination
    }

    pub fn emit_store(&mut self, ctx: &mut HIRContext, destination_ptr: ValueId, value: Value) {
        let value_type = self.get_value_type(&value);
        let destination_ptr_type = self.get_value_id_type(&destination_ptr);

        if let TypeKind::Pointer(target_type) = destination_ptr_type.kind {
            if !self.check_is_assignable(&value_type, &target_type) {
                ctx.module_builder.errors.push(SemanticError {
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

        self.get_current_basic_block().instructions.push(Instruction::Store {
            destination_ptr,
            source_val: value,
        });
    }

    pub fn emit_load(&mut self, source_ptr: ValueId) -> ValueId {
        let ptr_type = self.get_value_id_type(&source_ptr);

        let destination_type = if let TypeKind::Pointer(t) = ptr_type.kind {
            *t
        } else {
            panic!("INTERNAL COMPILER ERROR: Expected source_ptr to be of Pointer<T> type");
        };

        let destination = self.new_value_id();

        self.cfg.value_types.insert(destination, destination_type);

        self.get_current_basic_block()
            .instructions
            .push(Instruction::Load { destination, source_ptr });

        destination
    }

    pub fn emit_get_field_ptr(&mut self, base_ptr: ValueId, field: IdentifierNode) -> Result<ValueId, SemanticError> {
        let base_ptr_type = self.get_value_id_type(&base_ptr);

        let struct_decl = if let TypeKind::Pointer(ptr_to) = &base_ptr_type.kind {
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

        if let Some((field_index, checked_field)) = struct_decl
            .fields
            .iter()
            .enumerate()
            .find(|(_, f)| f.identifier.name == field.name)
        {
            let destination = self.new_value_id();
            let field_type = &checked_field.constraint;

            self.cfg.value_types.insert(
                destination,
                Type {
                    kind: TypeKind::Pointer(Box::new(field_type.clone())),
                    span: field.span,
                },
            );

            self.get_current_basic_block().instructions.push(Instruction::GetFieldPtr {
                destination,
                base_ptr,
                field_index,
            });

            Ok(destination)
        } else {
            Err(SemanticError {
                kind: SemanticErrorKind::AccessToUndefinedField { field },
                span: field.span,
            })
        }
    }

    pub fn emit_get_element_ptr(&mut self, module_builder: &mut ModuleBuilder) {
        todo!()
    }

    pub fn emit_unary_op(&mut self, ctx: &mut HIRContext, op_kind: UnaryOperationKind, value: Value) -> ValueId {
        let value_type = self.get_value_type(&value);
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

                    return self.report_error_and_get_poison(
                        ctx,
                        SemanticError {
                            kind: SemanticErrorKind::TypeMismatchExpectedOneOf {
                                expected,
                                received: value_type.clone(),
                            },
                            span,
                        },
                    );
                }

                self.new_value_id()
            }
            UnaryOperationKind::Not => {
                let bool_type = Type {
                    kind: TypeKind::Bool,
                    span,
                };

                if !self.check_is_assignable(&value_type, &bool_type) {
                    return self.report_error_and_get_poison(
                        ctx,
                        SemanticError {
                            kind: SemanticErrorKind::TypeMismatch {
                                expected: bool_type.clone(),
                                received: value_type,
                            },
                            span,
                        },
                    );
                }

                self.new_value_id()
            }
        };

        self.cfg.value_types.insert(destination, value_type);
        self.get_current_basic_block().instructions.push(Instruction::UnaryOp {
            op_kind,
            destination,
            operand: value,
        });

        destination
    }

    pub fn emit_binary_op(&mut self, module_builder: &mut ModuleBuilder) {
        todo!()
    }

    pub fn emit_phi(&mut self, module_builder: &mut ModuleBuilder) {
        todo!()
    }

    pub fn emit_function_call(&mut self, module_builder: &mut ModuleBuilder) {
        todo!()
    }

    pub fn emit_type_cast(&mut self, ctx: &mut HIRContext, value: Value, target_type: Type) -> ValueId {
        let value_type = self.get_value_type(&value);

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

        let destination = self.new_value_id();
        self.cfg.value_types.insert(destination, target_type.clone());

        self.get_current_basic_block().instructions.push(Instruction::TypeCast {
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
