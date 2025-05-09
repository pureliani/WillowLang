use std::hash::{Hash, Hasher};

use crate::{ast::IdentifierNode, parse::DocAnnotation};

use super::{checked_expression::CheckedExpr, checked_type::CheckedType};

#[derive(Clone, Debug)]
pub struct CheckedParam {
    pub identifier: IdentifierNode,
    pub constraint: CheckedType,
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
pub struct CheckedGenericParam {
    pub identifier: IdentifierNode,
    pub constraint: Option<Box<CheckedType>>,
}

impl Eq for CheckedGenericParam {}
impl PartialEq for CheckedGenericParam {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier && self.constraint == other.constraint
    }
}
impl Hash for CheckedGenericParam {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.constraint.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct GenericStructDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub generic_params: Vec<CheckedGenericParam>,
    pub properties: Vec<CheckedParam>,
}

impl Eq for GenericStructDecl {}
impl PartialEq for GenericStructDecl {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
            && self.generic_params == other.generic_params
            && self.properties == other.properties
    }
}
impl Hash for GenericStructDecl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.generic_params.hash(state);
        self.properties.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct StructDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub properties: Vec<CheckedParam>,
}

impl Eq for StructDecl {}
impl PartialEq for StructDecl {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier && self.properties == other.properties
    }
}
impl Hash for StructDecl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.properties.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct GenericTypeAliasDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub generic_params: Vec<CheckedGenericParam>,
    pub value: Box<CheckedType>,
}

impl Eq for GenericTypeAliasDecl {}
impl PartialEq for GenericTypeAliasDecl {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
            && self.generic_params == other.generic_params
            && self.value == other.value
    }
}
impl Hash for GenericTypeAliasDecl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.generic_params.hash(state);
        self.value.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct TypeAliasDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub value: Box<CheckedType>,
}

impl Eq for TypeAliasDecl {}
impl PartialEq for TypeAliasDecl {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier && self.value == other.value
    }
}
impl Hash for TypeAliasDecl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.value.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct CheckedVarDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub constraint: CheckedType,
    pub value: Option<CheckedExpr>,
}
