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
    Tag {
        identifier: IdentifierNode,
        value: Option<Box<Expr>>,
    },
    FnCall {
        left: Box<Expr>,
        args: Vec<Expr>,
    },
    StructLiteral {
        fields: Vec<(IdentifierNode, Expr)>,
    },
    BoolLiteral {
        value: bool,
    },
    Number {
        value: NumberKind,
    },
    String {
        value: StringNode,
    },
    Identifier {
        identifier: IdentifierNode,
    },
    Fn {
        name: IdentifierNode,
        params: Vec<Param>,
        return_type: Option<TypeAnnotation>,
        body: BlockContents,
    },
    If {
        condition: Box<Expr>,
        then_branch: BlockContents,
        else_if_branches: Vec<(Box<Expr>, BlockContents)>,
        else_branch: Option<BlockContents>,
    },
    ListLiteral {
        items: Vec<Expr>,
    },
    CodeBlock(BlockContents),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}
