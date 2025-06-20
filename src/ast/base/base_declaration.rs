use std::hash::{Hash, Hasher};

use crate::{
    ast::{IdentifierNode, Span},
    parse::DocAnnotation,
};

use super::{base_expression::Expr, base_type::TypeAnnotation};

#[derive(Clone, Debug, PartialEq)]
pub struct Param {
    pub identifier: IdentifierNode,
    pub constraint: TypeAnnotation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GenericParam {
    pub identifier: IdentifierNode,
    pub constraint: Option<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StructDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub generic_params: Vec<GenericParam>,
    pub fields: Vec<Param>,
}

#[derive(Clone, Debug)]
pub struct EnumDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub variants: Vec<IdentifierNode>,
    pub span: Span,
}

impl Eq for EnumDecl {}
impl PartialEq for EnumDecl {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier && self.variants == other.variants
    }
}
impl Hash for EnumDecl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.variants.hash(state);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeAliasDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub generic_params: Vec<GenericParam>,
    pub value: TypeAnnotation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VarDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub constraint: Option<TypeAnnotation>,
    pub value: Option<Expr>,
}
