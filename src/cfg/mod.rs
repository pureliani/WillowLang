use std::collections::HashMap;

use crate::{
    ast::{
        checked::{
            checked_declaration::{CheckedGenericParam, CheckedParam},
            checked_type::CheckedTypeKind,
        },
        DefinitionId, IdentifierNode, Span,
    },
    compile::string_interner::InternerId,
    tokenize::NumberKind,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BasicBlockId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AllocationSiteId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValueId(usize);

#[derive(Clone, Debug)]
pub enum RValue {
    NullLiteral {
        ty: CheckedTypeKind,
        span: Span,
    },
    BoolLiteral {
        value: bool,
        ty: CheckedTypeKind,
        span: Span,
    },
    NumberLiteral {
        value: NumberKind,
        ty: CheckedTypeKind,
        span: Span,
    },
    StringLiteral {
        value: InternerId,
        ty: CheckedTypeKind,
        span: Span,
    },
    Temp {
        id: ValueId,
        ty: CheckedTypeKind,
    },
}

#[derive(Clone, Debug)]
pub enum Pointer {
    StackSlot {
        id: DefinitionId,
        ty: CheckedTypeKind,
        span: Span,
    },
    FieldAccess {
        object_ptr: ValueId,
        object_type: CheckedTypeKind,
        field_name: IdentifierNode,
        field_type: CheckedTypeKind,
        span: Span,
    },
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Alloc {
        destination_ptr: Pointer,
        span: Span,
    },
    Store {
        destination_ptr: Pointer,
        source_val: RValue,
        span: Span,
    },
    Load {
        destination_temp: ValueId,
        source_ptr: Pointer,
        span: Span,
    },
    New {
        destination_temp: ValueId,
        result_type: CheckedTypeKind,
        allocation_site_id: AllocationSiteId,
        span: Span,
    },
    UnaryOp {
        op_kind: UnaryOperationKind,
        destination: ValueId,
        operand: RValue,
        result_type: CheckedTypeKind,
        span: Span,
    },
    BinaryOp {
        op_kind: BinaryOperationKind,
        destination: ValueId,
        left: RValue,
        right: RValue,
        result_type: CheckedTypeKind,
        span: Span,
    },
    TypeCast {
        destination: ValueId,
        operand: RValue,
        target_type: CheckedTypeKind,
        span: Span,
    },
    IsType {
        destination: ValueId,
        operand_to_check: RValue,
        type_to_check_against: CheckedTypeKind,
        span: Span,
    },
    FunctionCall {
        destination: Option<ValueId>,
        function_rvalue: RValue,
        args: Vec<RValue>,
        result_type: CheckedTypeKind,
        span: Span,
    },
    Phi {
        destination: ValueId,
        sources: Vec<(BasicBlockId, RValue)>,
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
        condition: RValue,
        true_target: BasicBlockId,
        false_target: BasicBlockId,
        span: Span,
    },
    Return {
        value: Option<RValue>,
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
    pub return_type: CheckedTypeKind,
    pub entry_block: BasicBlockId,
    pub blocks: HashMap<BasicBlockId, BasicBlock>,
    pub value_types: HashMap<ValueId, CheckedTypeKind>,
}
