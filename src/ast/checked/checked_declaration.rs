use crate::{ast::IdentifierNode, parse::DocAnnotation};

use super::{checked_expression::CheckedExpr, checked_type::CheckedType};

#[derive(Clone, Debug)]
pub struct CheckedParam {
    pub identifier: IdentifierNode,
    pub constraint: CheckedType,
}

#[derive(Clone, Debug)]
pub struct CheckedGenericParam {
    pub identifier: IdentifierNode,
    pub constraint: Option<Box<CheckedType>>,
}

#[derive(Clone, Debug)]
pub struct GenericStructDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub generic_params: Vec<CheckedGenericParam>,
    pub properties: Vec<CheckedParam>,
}

#[derive(Clone, Debug)]
pub struct StructDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub properties: Vec<CheckedParam>,
}

#[derive(Clone, Debug)]
pub struct GenericTypeAliasDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub generic_params: Vec<CheckedGenericParam>,
    pub value: Box<CheckedType>,
}

#[derive(Clone, Debug)]
pub struct TypeAliasDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub value: Box<CheckedType>,
}

#[derive(Clone, Debug)]
pub struct CheckedVarDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub constraint: CheckedType,
    pub value: Option<CheckedExpr>,
}
