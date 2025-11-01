use crate::{
    ast::{expr::BlockContents, IdentifierNode},
    parse::DocAnnotation,
};

use super::{expr::Expr, type_annotation::TypeAnnotation};

#[derive(Clone, Debug, PartialEq)]
pub struct Param {
    pub identifier: IdentifierNode,
    pub constraint: TypeAnnotation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FnDecl {
    pub identifier: IdentifierNode,
    pub params: Vec<Param>,
    pub return_type: TypeAnnotation,
    pub body: BlockContents,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeAliasDecl {
    pub documentation: Option<DocAnnotation>,
    pub identifier: IdentifierNode,
    pub value: TypeAnnotation,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Declaration {
    TypeAlias(TypeAliasDecl),
    Fn(FnDecl),
}

#[derive(Clone, Debug, PartialEq)]
pub struct VarDecl {
    pub documentation: Option<DocAnnotation>,
    pub identifier: IdentifierNode,
    pub constraint: Option<TypeAnnotation>,
    pub value: Option<Expr>,
}
