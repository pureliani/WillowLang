use crate::{ast::IdentifierNode, parse::DocAnnotation};

use super::{expr::Expr, type_annotation::TypeAnnotation};

#[derive(Clone, Debug, PartialEq)]
pub struct Param {
    pub identifier: IdentifierNode,
    pub constraint: TypeAnnotation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnumDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub generic_params: Vec<GenericParam>,
    pub variants: Vec<(IdentifierNode, Option<TypeAnnotation>)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GenericParam {
    pub identifier: IdentifierNode,
    pub constraint: Option<TypeAnnotation>,
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
