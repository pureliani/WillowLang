use crate::ast::{base::base_declaration::EnumDecl, IdentifierNode, Span, StringNode};

use super::{
    checked_declaration::{
        CheckedVarDecl, GenericStructDecl, GenericTypeAliasDecl, StructDecl, TypeAliasDecl,
    },
    checked_expression::{CheckedBlockContents, CheckedExpr},
};

#[derive(Clone, Debug)]
pub enum CheckedStmtKind {
    Expression(CheckedExpr),
    GenericStructDecl(GenericStructDecl),
    StructDecl(StructDecl),
    EnumDecl(EnumDecl),
    GenericTypeAliasDecl(GenericTypeAliasDecl),
    TypeAliasDecl(TypeAliasDecl),
    VarDecl(CheckedVarDecl),
    Break,
    Continue,
    Return(CheckedExpr),
    Assignment {
        target: CheckedExpr,
        value: CheckedExpr,
    },
    From {
        path: StringNode,
        identifiers: Vec<(IdentifierNode, Option<IdentifierNode>)>, // optional alias
    },
    While {
        condition: Box<CheckedExpr>,
        body: CheckedBlockContents,
    },
}

#[derive(Clone, Debug)]
pub struct CheckedStmt {
    pub kind: CheckedStmtKind,
    pub span: Span,
}
