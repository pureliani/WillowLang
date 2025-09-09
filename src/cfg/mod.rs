use std::collections::{HashMap, HashSet};

use crate::{
    ast::IdentifierNode,
    compile::string_interner::InternerId,
    hir_builder::types::{
        checked_declaration::{CheckedStructDecl, CheckedTypeAliasDecl, CheckedVarDecl},
        checked_type::{Type, TypeKind},
    },
    tokenize::NumberKind,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ModuleId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FunctionId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BasicBlockId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HeapAllocationId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValueId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ConstantId(pub usize);

#[derive(Clone, Debug)]
pub enum Value {
    VoidLiteral,
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
    pub entry_block: BasicBlockId,
    pub blocks: HashMap<BasicBlockId, BasicBlock>,
    pub value_types: HashMap<ValueId, Type>,
}

#[derive(Clone, Debug)]
pub enum CheckedDeclaration {
    TypeAliasDecl(CheckedTypeAliasDecl),
    StructDecl(CheckedStructDecl),
    UnionDecl(CheckedVarDecl),
}

#[derive(Clone, Debug)]
pub struct CheckedModule {
    pub id: ModuleId,
    pub name: String,
    pub functions: HashMap<FunctionId, ControlFlowGraph>,
    pub constant_data: HashMap<ConstantId, Vec<u8>>, // map: id -> bytes
    pub declarations: HashMap<IdentifierNode, CheckedDeclaration>,
    pub exports: HashSet<IdentifierNode>,
}
