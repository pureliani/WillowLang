use std::collections::HashMap;

use crate::{
    ast::Span,
    compile::string_interner::InternerId,
    hir_builder::types::{
        checked_declaration::{CheckedGenericParam, CheckedParam},
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

#[derive(Clone, Debug)]
pub enum Instruction {
    Alloc {
        destination: ValueId,
        ty: TypeKind,
        span: Span,
    },
    New {
        destination: ValueId,
        allocation_site_id: HeapAllocationId,
        ty: TypeKind,
        span: Span,
    },
    Store {
        destination_ptr: ValueId,
        source_val: Value,
        span: Span,
    },
    Load {
        destination: ValueId,
        source_ptr: ValueId,
        span: Span,
    },
    FieldPtr {
        destination: ValueId,
        base_ptr: ValueId,
        field_index: usize,
        span: Span,
    },
    ElementPtr {
        destination: ValueId,
        base_ptr: ValueId,
        index: Value,
        span: Span,
    },
    UnaryOp {
        op_kind: UnaryOperationKind,
        destination: ValueId,
        operand: Value,
        result_type: TypeKind,
        span: Span,
    },
    BinaryOp {
        op_kind: BinaryOperationKind,
        destination: ValueId,
        left: Value,
        right: Value,
        result_type: TypeKind,
        span: Span,
    },
    TypeCast {
        destination: ValueId,
        operand: Value,
        target_type: TypeKind,
        span: Span,
    },
    FunctionCall {
        destination: Option<ValueId>,
        function_rvalue: Value,
        args: Vec<Value>,
        span: Span,
    },
    Phi {
        destination: ValueId,
        sources: Vec<(BasicBlockId, Value)>,
    },
    Nop {
        span: Span,
    },
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
        span: Span,
    },
    CondJump {
        condition: Value,
        true_target: BasicBlockId,
        false_target: BasicBlockId,
        span: Span,
    },
    Return {
        value: Option<Value>,
        span: Span,
    },
    Unreachable {
        span: Span,
    },
}

#[derive(Clone, Debug)]
pub struct BasicBlock {
    pub id: BasicBlockId,
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
}

#[derive(Clone, Debug)]
pub struct ControlFlowGraph {
    pub generic_params: Vec<CheckedGenericParam>,
    pub parms: Vec<CheckedParam>,
    pub return_type: TypeKind,
    pub entry_block: BasicBlockId,
    pub blocks: HashMap<BasicBlockId, BasicBlock>,
    pub value_types: HashMap<ValueId, Type>,
}
