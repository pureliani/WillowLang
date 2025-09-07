use crate::ast::{
    decl::{StructDecl, UnionDecl},
    IdentifierNode, Span, StringNode,
};

use super::{
    decl::{TypeAliasDecl, VarDecl},
    expr::{BlockContents, Expr},
};

#[derive(Clone, Debug, PartialEq)]
pub enum StmtKind {
    Expression(Expr),
    TypeAliasDecl(TypeAliasDecl),
    VarDecl(VarDecl),
    StructDecl(StructDecl),
    UnionDecl(UnionDecl),
    Break,
    Continue,
    Return {
        value: Expr,
    },
    Assignment {
        target: Expr,
        value: Expr,
    },
    From {
        path: StringNode,
        identifiers: Vec<(IdentifierNode, Option<IdentifierNode>)>, // optional alias
    },
    While {
        condition: Box<Expr>,
        body: BlockContents,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}
