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
pub struct TemporaryId(usize);

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
        id: TemporaryId,
        ty: CheckedTypeKind,
    },
    Var {
        id: DefinitionId,
        ty: CheckedTypeKind,
        span: Span,
    },
}

#[derive(Clone, Debug)]
pub enum LValue {
    Var {
        id: DefinitionId,
        ty: CheckedTypeKind,
        span: Span,
    },
    FieldAccess {
        object_temp: TemporaryId,
        object_type: CheckedTypeKind,
        field_name: IdentifierNode,
        field_type: CheckedTypeKind,
        span: Span,
    },
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Assign {
        destination: LValue,
        source: RValue,
        span: Span,
    },

    // --- Heap Allocation ---
    // `New` allocates raw memory and returns a pointer (Temp) to it.
    // This pointer is of a "raw pointer" to the `result_type` type.
    New {
        destination: TemporaryId,
        result_type: CheckedTypeKind,
        allocation_site_id: AllocationSiteId,
        span: Span,
    },

    // --- Operations that produce a new value (into a TemporaryId) ---
    Load {
        destination: TemporaryId,
        source: LValue,
        span: Span,
    },
    UnaryOp {
        op_kind: UnaryOperationKind,
        destination: TemporaryId,
        operand: RValue,
        result_type: CheckedTypeKind,
        span: Span,
    },
    BinaryOp {
        op_kind: BinaryOperationKind,
        destination: TemporaryId,
        left: RValue,
        right: RValue,
        result_type: CheckedTypeKind,
        span: Span,
    },
    TypeCast {
        destination: TemporaryId,
        operand: RValue,
        target_type: CheckedTypeKind,
        span: Span,
    },
    IsType {
        destination: TemporaryId,
        operand_to_check: RValue,
        type_to_check_against: CheckedTypeKind,
        // result_type is implicitly Bool
        span: Span,
    },
    FunctionCall {
        destination: Option<TemporaryId>, // For non-void functions
        function_rvalue: RValue, // RValue that evaluates to a function (e.g., Var holding func, or direct func identifier)
        args: Vec<RValue>,
        result_type: CheckedTypeKind,
        span: Span,
    },
    Phi {
        destination: TemporaryId,
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
    pub variable_types: HashMap<(BasicBlockId, DefinitionId), CheckedTypeKind>,
}
