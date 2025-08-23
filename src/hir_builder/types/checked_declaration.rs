use std::hash::{Hash, Hasher};

use crate::{
    ast::{IdentifierNode, ModuleId, Span},
    cfg::ValueId,
    hir_builder::types::checked_type::Type,
    parse::DocAnnotation,
};

#[derive(Clone, Debug)]
pub struct CheckedTag {
    pub identifier: IdentifierNode,
    pub value_type: Option<Box<Type>>,
}

impl Eq for CheckedTag {}
impl PartialEq for CheckedTag {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier && self.value_type == other.value_type
    }
}
impl Hash for CheckedTag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.value_type.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct CheckedParam {
    pub identifier: IdentifierNode,
    pub constraint: Type,
}

impl Eq for CheckedParam {}
impl PartialEq for CheckedParam {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier && self.constraint == other.constraint
    }
}
impl Hash for CheckedParam {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.constraint.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct CheckedFnType {
    pub params: Vec<CheckedParam>,
    pub return_type: Box<Type>,
    pub span: Span,
}

impl Eq for CheckedFnType {}
impl PartialEq for CheckedFnType {
    fn eq(&self, other: &Self) -> bool {
        self.params == other.params && self.return_type == other.return_type
    }
}
impl Hash for CheckedFnType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.params.hash(state);
        self.return_type.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct CheckedTypeAliasDecl {
    pub identifier: IdentifierNode,
    pub module_id: ModuleId,
    pub documentation: Option<DocAnnotation>,
    pub value: Box<Type>,
    pub span: Span,
}

impl Eq for CheckedTypeAliasDecl {}
impl PartialEq for CheckedTypeAliasDecl {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier && self.module_id == other.module_id
    }
}
impl Hash for CheckedTypeAliasDecl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.module_id.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct CheckedVarDecl {
    pub value_id: ValueId,
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub constraint: Type,
}
