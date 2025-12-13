use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use crate::{ast::IdentifierNode, hir::types::checked_type::Type, tokenize::NumberKind};

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
    LoadConstant {
        destination: ValueId,
        constant_id: ConstantId,
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
pub struct CheckedModule {
    pub path: PathBuf,
    pub exports: HashSet<IdentifierNode>,
}

impl CheckedModule {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            exports: HashSet::new(),
        }
    }
}
