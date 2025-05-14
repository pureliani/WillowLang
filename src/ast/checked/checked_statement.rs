use crate::ast::{base::base_declaration::EnumDecl, IdentifierNode, Span, StringNode};

use super::{
    checked_declaration::{
        CheckedGenericStructDecl, CheckedGenericTypeAliasDecl, CheckedStructDecl,
        CheckedTypeAliasDecl, CheckedVarDecl,
    },
    checked_expression::{CheckedBlockContents, CheckedExpr},
};

#[derive(Clone, Debug)]
pub enum CheckedStmtKind {
    Expression(CheckedExpr),
    GenericStructDecl(CheckedGenericStructDecl),
    StructDecl(CheckedStructDecl),
    EnumDecl(EnumDecl),
    GenericTypeAliasDecl(CheckedGenericTypeAliasDecl),
    TypeAliasDecl(CheckedTypeAliasDecl),
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
