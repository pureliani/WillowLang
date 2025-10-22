use std::hash::{Hash, Hasher};

use crate::{
    ast::{IdentifierNode, Span},
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
pub struct CheckedTagType {
    pub identifier: IdentifierNode,
    pub value_type: Option<Box<Type>>,
    pub span: Span,
}

impl Eq for CheckedTagType {}
impl PartialEq for CheckedTagType {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier && self.value_type == other.value_type
    }
}
impl Hash for CheckedTagType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.value_type.hash(state);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CheckedFnType {
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
    /// The variable is on the stack, ValueId is a direct pointer to its stack slot.
    Stack(ValueId),
    /// The variable has been captured by a closure.
    Captured,
}

#[derive(Clone, Debug)]
pub struct CheckedVarDecl {
    pub id: DeclarationId,
    pub storage: VarStorage,
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub constraint: Type,
}
