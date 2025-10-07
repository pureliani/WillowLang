use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use crate::{
    ast::IdentifierNode,
    compile::string_interner::InternerId,
    hir::types::{
        checked_declaration::{CheckedEnumDecl, CheckedTypeAliasDecl, CheckedVarDecl},
        checked_type::Type,
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
pub enum IntrinsicFunction {
    ListSet {
        list_base_ptr: ValueId,
        index: Value,
        item: Value,
    },
    ListGet {
        list_base_ptr: ValueId,
        index: Value,
        destination: ValueId,
    },
}

#[derive(Clone, Debug)]
pub enum IntrinsicField {
    ListLen { list_base_ptr: ValueId, destination: ValueId },
}

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
pub enum PtrOffset {
    // A dynamic offset, calculated at runtime.
    Dynamic(Value),
    // A constant offset, known at compile time.
    Constant(usize),
}

#[derive(Clone, Debug)]
pub enum Instruction {
    StackAlloc {
        destination: ValueId,
        count: usize,
    },
    HeapAlloc {
        destination: ValueId,
        allocation_site_id: HeapAllocationId,
        count: Value,
    },
    Store {
        destination_ptr: ValueId,
        source_val: Value,
    },
    Load {
        destination: ValueId,
        source_ptr: ValueId,
    },
    GetFieldPtr {
        destination: ValueId,
        base_ptr: ValueId,
        field_index: usize,
    },
    PtrAdd {
        destination: ValueId,
        base_ptr: ValueId,
        // The index is now this more explicit enum.
        offset: PtrOffset,
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
        target_type: Type,
    },
    FunctionCall {
        destination: Option<ValueId>,
        function_rvalue: Value,
        args: Vec<Value>,
    },
    IntrinsicFunctionCall(IntrinsicFunction),
    IntrinsicFieldAccess(IntrinsicField),
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
}

#[derive(Clone, Debug)]
pub enum CheckedDeclaration {
    TypeAliasDecl(CheckedTypeAliasDecl),
    VarDecl(CheckedVarDecl),
    EnumDecl(CheckedEnumDecl),
}

#[derive(Clone, Debug)]
pub struct CheckedModule {
    pub id: ModuleId,
    pub name: PathBuf,
    pub functions: HashMap<FunctionId, ControlFlowGraph>,
    pub constant_data: HashMap<ConstantId, Vec<u8>>, // map: id -> bytes
    pub declarations: HashMap<IdentifierNode, CheckedDeclaration>,
    pub exports: HashSet<IdentifierNode>,
}

impl CheckedModule {
    pub fn new(id: ModuleId, name: PathBuf) -> Self {
        Self {
            id,
            name,
            declarations: HashMap::new(),
            constant_data: HashMap::new(),
            exports: HashSet::new(),
            functions: HashMap::new(),
        }
    }
}
