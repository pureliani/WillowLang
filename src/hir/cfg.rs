use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::{Arc, RwLock},
};

use crate::{
    ast::IdentifierNode,
    hir::types::{
        checked_declaration::{CheckedTypeAliasDecl, CheckedVarDecl},
        checked_type::Type,
    },
    tokenize::NumberKind,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FunctionId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BasicBlockId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValueId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ConstantId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DeclarationId(pub usize);

#[derive(Clone, Debug)]
pub enum Value {
    VoidLiteral,
    BoolLiteral(bool),
    NumberLiteral(NumberKind),
    StringLiteral(String),
    /// Represents a reference to a function.
    Function {
        function_id: FunctionId,
        ty: Type,
    },
    Use(ValueId),
}

#[derive(Clone, Debug)]
pub enum Instruction {
    StackAlloc {
        destination: ValueId,
        count: usize,
    },
    HeapAlloc {
        destination: ValueId,
        count: Value,
    },
    HeapFree {
        ptr: ValueId,
    },
    Store {
        ptr: ValueId,
        value: Value,
    },
    Load {
        destination: ValueId,
        ptr: ValueId,
    },
    FileOpen {
        destination_fd: ValueId,
        path: Value,
        mode: Value,
    },
    FileWrite {
        fd: ValueId,
        data: Value,
        len: Value,
    },
    FileRead {
        fd: ValueId,
        buffer: ValueId,
        len: Value,
        value_destination: ValueId, // Result of read()
    },
    FileClose {
        fd: ValueId,
    },
    SocketConnect {
        address: Value,
        port: Value,
        value_destination: ValueId,
    },
    SocketSend {
        socket: ValueId,
        data: Value,
    },
    SocketClose {
        socket: ValueId,
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
    GetFieldPtr {
        destination: ValueId,
        base_ptr: ValueId,
        field_index: usize,
    },
    GetElementPtr {
        destination: ValueId,
        base_ptr: ValueId,
        index: Value,
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
        args: Vec<Value>,
    },
    CondJump {
        condition: Value,
        true_target: BasicBlockId,
        true_args: Vec<Value>,
        false_target: BasicBlockId,
        false_args: Vec<Value>,
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
    pub terminator: Option<Terminator>,
    pub params: Vec<ValueId>,
}

#[derive(Clone, Debug)]
pub struct ControlFlowGraph {
    pub entry_block: BasicBlockId,
    pub blocks: HashMap<BasicBlockId, BasicBlock>,
}

#[derive(Clone, Debug)]
pub enum CheckedDeclaration {
    TypeAlias(Arc<RwLock<CheckedTypeAliasDecl>>),
    Function(FunctionId),
    Var(CheckedVarDecl),
    // This is for detecting the Temporal Dead Zone
    UninitializedVar { identifier: IdentifierNode },
}

#[derive(Clone, Debug)]
pub struct CheckedModule {
    pub path: PathBuf,
    pub functions: HashMap<FunctionId, ControlFlowGraph>,
    pub constant_data: HashMap<ConstantId, Vec<u8>>,
    pub declarations: HashMap<IdentifierNode, CheckedDeclaration>,
    pub exports: HashSet<IdentifierNode>,
}

impl CheckedModule {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            declarations: HashMap::new(),
            constant_data: HashMap::new(),
            exports: HashSet::new(),
            functions: HashMap::new(),
        }
    }
}
