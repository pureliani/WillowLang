use crate::ast::{base::base_declaration::EnumDecl, IdentifierNode, Span, StringNode};

use super::{
    monomorphized_declaration::{MonoStructDecl, MonoTypeAliasDecl, MonoVarDecl},
    monomorphized_expression::{MonoBlockContents, MonoExpr},
};

#[derive(Clone, Debug)]
pub enum MonoStmtKind {
    Expression(MonoExpr),
    StructDecl(MonoStructDecl),
    EnumDecl(EnumDecl),
    TypeAliasDecl(MonoTypeAliasDecl),
    VarDecl(MonoVarDecl),
    Break,
    Continue,
    Return(MonoExpr),
    Assignment {
        target: MonoExpr,
        value: MonoExpr,
    },
    From {
        path: StringNode,
        identifiers: Vec<(IdentifierNode, Option<IdentifierNode>)>, // optional alias
    },
    While {
        condition: Box<MonoExpr>,
        body: MonoBlockContents,
    },
}

#[derive(Clone, Debug)]
pub struct MonoStmt {
    pub kind: MonoStmtKind,
    pub span: Span,
}
