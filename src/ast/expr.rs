use crate::{
    ast::{IdentifierNode, Span, StringNode},
    tokenize::NumberKind,
};

use super::{decl::Param, stmt::Stmt, type_annotation::TypeAnnotation};

#[derive(Clone, Debug, PartialEq)]
pub struct BlockContents {
    pub statements: Vec<Stmt>,
    pub final_expr: Option<Box<Expr>>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MatchPattern {
    VariantWithValue(IdentifierNode, IdentifierNode), // e.g: match ... { Some(v) => .. }
    Variant(IdentifierNode),                          // e.g match ... { None => .. }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MatchArm {
    pub pattern: Vec<MatchPattern>, // e.g match x, y, z { Foo(x), Bar, Baz(y) => {} }
    pub expression: Expr,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprKind {
    Not {
        right: Box<Expr>,
    },
    Neg {
        right: Box<Expr>,
    },
    Add {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Subtract {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Multiply {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Divide {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Modulo {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    LessThan {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    LessThanOrEqual {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    GreaterThan {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    GreaterThanOrEqual {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Equal {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    NotEqual {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    And {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Or {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    StructInit(Vec<(IdentifierNode, Expr)>),
    Access {
        left: Box<Expr>,
        field: IdentifierNode,
    },
    StaticAccess {
        left: Box<Expr>,
        field: IdentifierNode,
    },
    TypeCast {
        left: Box<Expr>,
        target: TypeAnnotation,
    },
    FnCall {
        left: Box<Expr>,
        args: Vec<Expr>,
    },
    BoolLiteral(bool),
    Number(NumberKind),
    String(StringNode),
    Identifier(IdentifierNode),
    Fn {
        name: IdentifierNode,
        params: Vec<Param>,
        return_type: TypeAnnotation,
        body: BlockContents,
    },
    Match {
        conditions: Vec<Expr>,
        arms: Vec<MatchArm>,
    },
    If {
        branches: Vec<(Box<Expr>, BlockContents)>,
        else_branch: Option<BlockContents>,
    },
    ArrayLiteral(Vec<Expr>),
    CodeBlock(BlockContents),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}
