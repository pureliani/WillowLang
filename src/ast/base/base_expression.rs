use crate::{
    ast::{IdentifierNode, Span, StringNode},
    parse::ParsingError,
    tokenizer::NumberKind,
};

use super::{
    base_declaration::{GenericParam, Param},
    base_statement::Stmt,
    base_type::TypeAnnotation,
};

#[derive(Clone, Debug, PartialEq)]
pub struct BlockContents {
    pub statements: Vec<Stmt>,
    pub final_expr: Option<Box<Expr>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprKind {
    // Prefix expressions
    Not {
        right: Box<Expr>,
    },
    Neg {
        right: Box<Expr>,
    },
    // Infix expressions
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
    // Suffix expressions
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
    IsType {
        left: Box<Expr>,
        target: TypeAnnotation,
    },
    GenericApply {
        left: Box<Expr>,
        args: Vec<TypeAnnotation>,
    },
    FnCall {
        left: Box<Expr>,
        args: Vec<Expr>,
    },
    StructInit {
        left: Box<Expr>,
        fields: Vec<(IdentifierNode, Expr)>,
    },
    // Basic/literal expressions
    Null,
    BoolLiteral {
        value: bool,
    },
    Number {
        value: NumberKind,
    },
    String(StringNode),
    Identifier(IdentifierNode),
    // Complex expressions
    Fn {
        params: Vec<Param>,
        body: BlockContents,
        return_type: Option<TypeAnnotation>,
        generic_params: Vec<GenericParam>,
    },
    If {
        condition: Box<Expr>,
        then_branch: BlockContents,
        else_if_branches: Vec<(Box<Expr>, BlockContents)>,
        else_branch: Option<BlockContents>,
    },
    ArrayLiteral {
        items: Vec<Box<Expr>>,
    },
    Block(BlockContents),
    Error(ParsingError),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}
