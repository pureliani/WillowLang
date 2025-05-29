use crate::{
    ast::{IdentifierNode, Span},
    compile::string_interner::InternerId,
};

use super::{
    base_declaration::{EnumDecl, StructDecl, TypeAliasDecl, VarDecl},
    base_expression::{BlockContents, Expr},
};

#[derive(Clone, Debug, PartialEq)]
pub enum StmtKind {
    Expression(Expr),
    StructDecl(StructDecl),
    EnumDecl(EnumDecl),
    TypeAliasDecl(TypeAliasDecl),
    VarDecl(VarDecl),
    Break,
    Continue,
    Return(Expr),
    Assignment {
        target: Expr,
        value: Expr,
    },
    From {
        path: InternerId,
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
