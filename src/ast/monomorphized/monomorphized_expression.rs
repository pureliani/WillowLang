use crate::{
    ast::{IdentifierNode, StringNode},
    tokenizer::NumberKind,
};

use super::{
    monomorphized_declaration::MonoParam, monomorphized_statement::MonoStmt,
    monomorphized_type::MonoType,
};

#[derive(Clone, Debug)]
pub struct MonoBlockContents {
    pub statements: Vec<MonoStmt>,
    pub final_expr: Option<Box<MonoExpr>>,
}

#[derive(Clone, Debug)]
pub enum MonoExprKind {
    // Prefix expressions
    Not {
        right: Box<MonoExpr>,
    },
    Neg {
        right: Box<MonoExpr>,
    },
    // Infix expressions
    Add {
        left: Box<MonoExpr>,
        right: Box<MonoExpr>,
    },
    Subtract {
        left: Box<MonoExpr>,
        right: Box<MonoExpr>,
    },
    Multiply {
        left: Box<MonoExpr>,
        right: Box<MonoExpr>,
    },
    Divide {
        left: Box<MonoExpr>,
        right: Box<MonoExpr>,
    },
    Modulo {
        left: Box<MonoExpr>,
        right: Box<MonoExpr>,
    },
    LessThan {
        left: Box<MonoExpr>,
        right: Box<MonoExpr>,
    },
    LessThanOrEqual {
        left: Box<MonoExpr>,
        right: Box<MonoExpr>,
    },
    GreaterThan {
        left: Box<MonoExpr>,
        right: Box<MonoExpr>,
    },
    GreaterThanOrEqual {
        left: Box<MonoExpr>,
        right: Box<MonoExpr>,
    },
    Equal {
        left: Box<MonoExpr>,
        right: Box<MonoExpr>,
    },
    NotEqual {
        left: Box<MonoExpr>,
        right: Box<MonoExpr>,
    },
    And {
        left: Box<MonoExpr>,
        right: Box<MonoExpr>,
    },
    Or {
        left: Box<MonoExpr>,
        right: Box<MonoExpr>,
    },
    // Suffix expressions
    Access {
        left: Box<MonoExpr>,
        field: IdentifierNode,
    },
    StaticAccess {
        left: Box<MonoExpr>,
        field: IdentifierNode,
    },
    TypeCast {
        left: Box<MonoExpr>,
        target: MonoType,
    },
    IsType {
        left: Box<MonoExpr>,
        target: MonoType,
    },
    FnCall {
        left: Box<MonoExpr>,
        args: Vec<MonoExpr>,
    },
    StructInit {
        left: Box<MonoExpr>,
        fields: Vec<(IdentifierNode, MonoExpr)>,
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
        params: Vec<MonoParam>,
        body: MonoBlockContents,
        return_type: MonoType,
    },
    If {
        condition: Box<MonoExpr>,
        then_branch: MonoBlockContents,
        else_if_branches: Vec<(Box<MonoExpr>, MonoBlockContents)>,
        else_branch: Option<MonoBlockContents>,
    },
    ArrayLiteral {
        items: Vec<MonoExpr>,
    },
    Block(MonoBlockContents),
}

#[derive(Clone, Debug)]
pub struct MonoExpr {
    pub kind: MonoExprKind,
    pub expr_type: MonoType,
}
