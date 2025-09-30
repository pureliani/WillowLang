use crate::{ast::IdentifierNode, parse::DocAnnotation};

use super::{expr::Expr, type_annotation::TypeAnnotation};

#[derive(Clone, Debug, PartialEq)]
pub struct Param {
    pub identifier: IdentifierNode,
    pub constraint: TypeAnnotation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeAliasDecl {
    pub documentation: Option<DocAnnotation>,
    pub identifier: IdentifierNode,
    pub value: TypeAnnotation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StructDecl {
    pub documentation: Option<DocAnnotation>,
    pub identifier: IdentifierNode,
    pub fields: Vec<Param>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnumDeclVariant {
    pub name: IdentifierNode,
    pub payload: Option<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnumDecl {
    pub documentation: Option<DocAnnotation>,
    pub identifier: IdentifierNode,
    pub variants: Vec<EnumDeclVariant>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VarDecl {
    pub documentation: Option<DocAnnotation>,
    pub identifier: IdentifierNode,
    pub constraint: Option<TypeAnnotation>,
    pub value: Option<Expr>,
}
