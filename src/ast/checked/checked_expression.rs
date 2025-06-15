use std::collections::HashSet;

use crate::{
    ast::{checked::checked_type::CheckedType, DefinitionId, IdentifierNode, StringNode},
    check::utils::substitute_generics::GenericSubstitutionMap,
    tfg::{TFGNodeVariableTypes, TypeFlowGraph},
    tokenize::NumberKind,
};

use super::{
    checked_declaration::{CheckedGenericParam, CheckedParam},
    checked_statement::CheckedStmt,
};

#[derive(Debug, Clone)]
pub struct FunctionSummary {
    pub exit_states: HashSet<TFGNodeVariableTypes>,
    pub guaranteed_calls: HashSet<DefinitionId>,
}

#[derive(Clone, Debug)]
pub struct CheckedBlockContents {
    pub statements: Vec<CheckedStmt>,
    pub final_expr: Option<Box<CheckedExpr>>,
}

#[derive(Clone, Debug)]
pub enum CheckedExprKind {
    // Prefix expressions
    Not {
        right: Box<CheckedExpr>,
    },
    Neg {
        right: Box<CheckedExpr>,
    },
    // Infix expressions
    Add {
        left: Box<CheckedExpr>,
        right: Box<CheckedExpr>,
    },
    Subtract {
        left: Box<CheckedExpr>,
        right: Box<CheckedExpr>,
    },
    Multiply {
        left: Box<CheckedExpr>,
        right: Box<CheckedExpr>,
    },
    Divide {
        left: Box<CheckedExpr>,
        right: Box<CheckedExpr>,
    },
    Modulo {
        left: Box<CheckedExpr>,
        right: Box<CheckedExpr>,
    },
    LessThan {
        left: Box<CheckedExpr>,
        right: Box<CheckedExpr>,
    },
    LessThanOrEqual {
        left: Box<CheckedExpr>,
        right: Box<CheckedExpr>,
    },
    GreaterThan {
        left: Box<CheckedExpr>,
        right: Box<CheckedExpr>,
    },
    GreaterThanOrEqual {
        left: Box<CheckedExpr>,
        right: Box<CheckedExpr>,
    },
    Equal {
        left: Box<CheckedExpr>,
        right: Box<CheckedExpr>,
    },
    NotEqual {
        left: Box<CheckedExpr>,
        right: Box<CheckedExpr>,
    },
    And {
        left: Box<CheckedExpr>,
        right: Box<CheckedExpr>,
    },
    Or {
        left: Box<CheckedExpr>,
        right: Box<CheckedExpr>,
    },
    // Suffix expressions
    Access {
        left: Box<CheckedExpr>,
        field: IdentifierNode,
    },
    StaticAccess {
        left: Box<CheckedExpr>,
        field: IdentifierNode,
    },
    TypeCast {
        left: Box<CheckedExpr>,
        target: CheckedType,
    },
    IsType {
        left: Box<CheckedExpr>,
        target: CheckedType,
    },
    FnCall {
        left: Box<CheckedExpr>,
        args: Vec<CheckedExpr>,
    },
    StructInit {
        left: Box<CheckedExpr>,
        fields: Vec<(IdentifierNode, CheckedExpr)>,
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
        id: DefinitionId,
        params: Vec<CheckedParam>,
        body: CheckedBlockContents,
        return_type: CheckedType,
        generic_params: Vec<CheckedGenericParam>,
        tfg: TypeFlowGraph,
        summary: FunctionSummary,
    },
    If {
        condition: Box<CheckedExpr>,
        then_branch: CheckedBlockContents,
        else_if_branches: Vec<(Box<CheckedExpr>, CheckedBlockContents)>,
        else_branch: Option<CheckedBlockContents>,
    },
    ArrayLiteral {
        items: Vec<CheckedExpr>,
    },
    TypeSpecialization {
        target: Box<CheckedExpr>,
        substitutions: GenericSubstitutionMap,
    },
    Block(CheckedBlockContents),
}

#[derive(Clone, Debug)]
pub struct CheckedExpr {
    pub kind: CheckedExprKind,
    pub ty: CheckedType,
}
