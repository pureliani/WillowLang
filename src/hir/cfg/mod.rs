pub mod basic_blocks;
pub mod instructions;

pub struct Place {
    /// The local variable (stack slot) this starts from
    pub root: ValueId,
    /// The sequence of projections (fields, index, deref)
    pub projections: Vec<Projection>,
}

pub enum Projection {
    Field(usize),
    Index(ValueId),
    Deref,
}

use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use crate::{
    compile::interner::StringId,
    hir::cfg::basic_blocks::{BasicBlock, BasicBlockId},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValueId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ConstantId(pub usize);

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
        args: Vec<ValueId>,
    },
    CondJump {
        condition: ValueId,
        true_target: BasicBlockId,
        true_args: Vec<ValueId>,
        false_target: BasicBlockId,
        false_args: Vec<ValueId>,
    },
    Return {
        value: Option<ValueId>,
    },
    Unreachable,
}

#[derive(Clone, Debug)]
pub struct ControlFlowGraph {
    pub entry_block: BasicBlockId,
    pub blocks: HashMap<BasicBlockId, BasicBlock>,
}

#[derive(Clone, Debug)]
pub struct CheckedModule {
    pub path: PathBuf,
    pub exports: HashSet<StringId>,
}

impl CheckedModule {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            exports: HashSet::new(),
        }
    }
}
