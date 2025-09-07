use crate::{ast::IdentifierNode, parse::DocAnnotation};

use super::{expr::Expr, type_annotation::TypeAnnotation};

#[derive(Clone, Debug, PartialEq)]
pub struct Param {
    pub identifier: IdentifierNode,
    pub constraint: TypeAnnotation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeAliasDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub value: TypeAnnotation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StructDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub fields: Vec<Param>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnionDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub variants: Vec<(IdentifierNode, Option<TypeAnnotation>)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VarDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub constraint: Option<TypeAnnotation>,
    pub value: Option<Expr>,
}
