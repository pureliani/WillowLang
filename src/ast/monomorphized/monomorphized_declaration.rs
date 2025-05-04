use crate::ast::IdentifierNode;

use super::{monomorphized_expression::MonoExpr, monomorphized_type::MonoType};

#[derive(Clone, Debug)]
pub struct MonoParam {
    pub identifier: IdentifierNode,
    pub constraint: MonoType,
}

#[derive(Clone, Debug)]
pub struct MonoStructDecl {
    pub identifier: IdentifierNode,
    pub properties: Vec<MonoParam>,
}

#[derive(Clone, Debug)]
pub struct MonoTypeAliasDecl {
    pub identifier: IdentifierNode,
    pub value: Box<MonoType>,
}

#[derive(Clone, Debug)]
pub struct MonoVarDecl {
    pub identifier: IdentifierNode,
    pub constraint: MonoType,
    pub value: Option<MonoExpr>,
}
