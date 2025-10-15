use std::hash::{Hash, Hasher};

use crate::{
    ast::{IdentifierNode, Span},
    hir::{cfg::ValueId, types::checked_type::Type},
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
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub value: Box<Type>,
    pub span: Span,
}

impl Eq for CheckedTypeAliasDecl {}
impl PartialEq for CheckedTypeAliasDecl {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier && self.value == other.value
    }
}
impl Hash for CheckedTypeAliasDecl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.value.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct CheckedVarDecl {
    pub stack_ptr: ValueId,
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub constraint: Type,
}
