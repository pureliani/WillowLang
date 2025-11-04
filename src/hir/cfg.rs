use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::{Arc, RwLock},
};

use crate::{
    ast::IdentifierNode,
    compile::interner::StringId,
    hir::types::{
        checked_declaration::{CheckedFnDecl, CheckedTypeAliasDecl, CheckedVarDecl},
        checked_type::Type,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ConstantId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DeclarationId(pub usize);

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
    ListLen {
        list_base_ptr: ValueId,
        destination: ValueId,
    },
}

#[derive(Clone, Debug)]
pub enum Value {
    VoidLiteral,
    BoolLiteral(bool),
    NumberLiteral(NumberKind),
    StringLiteral(StringId),
    FunctionAddr { function_id: FunctionId, ty: Type },
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
        allocation_site_id: HeapAllocationId,
        count: Value,
    },
    Free {
        pointer: ValueId,
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
    pub terminator: Option<Terminator>,
}

#[derive(Clone, Debug)]
pub struct ControlFlowGraph {
    pub entry_block: BasicBlockId,
    pub blocks: HashMap<BasicBlockId, BasicBlock>,
}

#[derive(Clone, Debug)]
pub enum CheckedDeclaration {
    TypeAlias(Arc<RwLock<CheckedTypeAliasDecl>>),
    Function(Arc<RwLock<CheckedFnDecl>>),
    Var(CheckedVarDecl),
}

#[derive(Clone, Debug)]
pub struct CheckedModule {
    pub path: PathBuf,
    pub functions: HashMap<FunctionId, ControlFlowGraph>,
    pub constant_data: HashMap<ConstantId, Vec<u8>>, // map id -> bytes
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
