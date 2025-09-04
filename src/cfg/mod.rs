use std::collections::HashMap;

use crate::{
    compile::string_interner::InternerId,
    hir_builder::types::{
        checked_declaration::CheckedParam,
        checked_type::{Type, TypeKind},
    },
    tokenize::NumberKind,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FunctionId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BasicBlockId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HeapAllocationId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValueId(pub usize);

#[derive(Clone, Debug)]
pub enum Value {
    BoolLiteral(bool),
    NumberLiteral(NumberKind),
    StringLiteral(InternerId),
    FunctionAddr { function_id: FunctionId, ty: Type },
    Use(ValueId),
}

impl Value {
    pub fn get_value_type(&self, value_types: &HashMap<ValueId, Type>) -> Type {
        match self {
            Value::BoolLiteral(_) => Type {
                kind: TypeKind::Bool,
                // TODO: fix this later
                span: Default::default(),
            },
            Value::NumberLiteral(kind) => {
                let kind = match kind {
                    NumberKind::I64(_) => TypeKind::I64,
                    NumberKind::I32(_) => TypeKind::I32,
                    NumberKind::I16(_) => TypeKind::I16,
                    NumberKind::I8(_) => TypeKind::I8,
                    NumberKind::F32(_) => TypeKind::F32,
                    NumberKind::F64(_) => TypeKind::F64,
                    NumberKind::U64(_) => TypeKind::U64,
                    NumberKind::U32(_) => TypeKind::U32,
                    NumberKind::U16(_) => TypeKind::U16,
                    NumberKind::U8(_) => TypeKind::U8,
                    NumberKind::USize(_) => TypeKind::USize,
                    NumberKind::ISize(_) => TypeKind::ISize,
                };

                Type {
                    kind,
                    span: Default::default(),
                }
            }
            Value::StringLiteral(_) => Type {
                kind: TypeKind::String,
                span: Default::default(),
            },
            Value::FunctionAddr { ty, .. } => ty.clone(),
            Value::Use(value_id) => value_types
                .get(value_id)
                .expect("INTERNAL: All ValueIds must have a corresponding type")
                .clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Alloc {
        destination: ValueId,
    },
    New {
        destination: ValueId,
        allocation_site_id: HeapAllocationId,
    },
    Store {
        destination_ptr: ValueId,
        source_val: Value,
    },
    Load {
        destination: ValueId,
        source_ptr: ValueId,
    },
    FieldPtr {
        destination: ValueId,
        base_ptr: ValueId,
        field_index: usize,
    },
    ElementPtr {
        destination: ValueId,
        base_ptr: ValueId,
        index: Value,
    },
    UnaryOp {
        op_kind: UnaryOperationKind,
        destination: ValueId,
        operand: Value,
    },
    BinaryOp {
        op_kind: BinaryOperationKind,
        destination: ValueId,
        left: Value,
        right: Value,
    },
    TypeCast {
        destination: ValueId,
        operand: Value,
        target_type: TypeKind,
    },
    FunctionCall {
        destination: Option<ValueId>,
        function_rvalue: Value,
        args: Vec<Value>,
    },
    Phi {
        destination: ValueId,
        sources: Vec<(BasicBlockId, Value)>,
    },
    Nop,
}

#[derive(Clone, Debug)]
pub enum UnaryOperationKind {
    Not,
    Neg,
}

#[derive(Clone, Debug)]
pub enum BinaryOperationKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
}

#[derive(Clone, Debug)]
pub enum Terminator {
    Jump {
        target: BasicBlockId,
    },
    CondJump {
        condition: Value,
        true_target: BasicBlockId,
        false_target: BasicBlockId,
    },
    Return {
        value: Option<Value>,
    },
    Unreachable,
}

#[derive(Clone, Debug)]
pub struct BasicBlock {
    pub id: BasicBlockId,
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
}

#[derive(Clone, Debug)]
pub struct ControlFlowGraph {
    pub parms: Vec<CheckedParam>,
    pub return_type: TypeKind,
    pub entry_block: BasicBlockId,
    pub blocks: HashMap<BasicBlockId, BasicBlock>,
    pub value_types: HashMap<ValueId, Type>,
}
