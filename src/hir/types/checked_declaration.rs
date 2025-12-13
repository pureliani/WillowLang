use std::hash::{Hash, Hasher};

use crate::{
    ast::{IdentifierNode, Span},
    compile::interner::TagId,
    hir::{
        cfg::{ControlFlowGraph, DeclarationId, ValueId},
        types::checked_type::Type,
    },
    parse::DocAnnotation,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CheckedParam {
    pub identifier: IdentifierNode,
    pub ty: Type,
}

#[derive(Clone, Debug)]
pub struct TagType {
    pub id: TagId,
    pub value_type: Option<Box<Type>>,
    pub span: Span,
}

impl Eq for TagType {}
impl PartialEq for TagType {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value_type == other.value_type
    }
}
impl Hash for TagType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.value_type.hash(state);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FnType {
    pub params: Vec<CheckedParam>,
    pub return_type: Box<Type>,
}

#[derive(Clone, Debug)]
pub struct CheckedTypeAliasDecl {
    pub id: DeclarationId,
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub value: Box<Type>,
    pub span: Span,
}

impl Eq for CheckedTypeAliasDecl {}
impl PartialEq for CheckedTypeAliasDecl {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Hash for CheckedTypeAliasDecl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.identifier.hash(state);
        self.value.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct CheckedFnDecl {
    pub id: DeclarationId,
    pub identifier: IdentifierNode,
    pub params: Vec<CheckedParam>,
    pub return_type: Type,
    pub body: Option<ControlFlowGraph>,
}

#[derive(Clone, Debug)]
pub enum VarStorage {
    /// The variable is a direct SSA value.
    /// The specific ValueId is tracked dynamically in FunctionBuilder.block_locals.
    Local,

    /// The variable is stored in memory (e.g., captured in a closure environment).
    /// The ValueId is a pointer to the storage location.
    Heap(ValueId),
}

#[derive(Clone, Debug)]
pub struct CheckedVarDecl {
    pub id: DeclarationId,
    pub storage: VarStorage,
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub constraint: Type,
}

#[derive(Clone, Debug)]
pub enum CheckedDeclaration {
    TypeAlias(CheckedTypeAliasDecl),
    Function(CheckedFnDecl),
    Var(CheckedVarDecl),
    // This is for detecting the Temporal Dead Zone
    UninitializedVar {
        id: DeclarationId,
        identifier: IdentifierNode,
    },
}
