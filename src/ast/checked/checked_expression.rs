use crate::{
    ast::{checked::checked_type::Type, DefinitionId, IdentifierNode, StringNode},
    check::utils::substitute_generics::GenericSubstitutionMap,
    tokenize::NumberKind,
};

use super::{
    checked_declaration::{CheckedGenericParam, CheckedParam},
    checked_statement::CheckedStmt,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LiteralValue {
    Bool(bool),
    Null,
    // String(InternerId),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RefinementKey {
    /// The refinement applies when the function returns a specific literal value.
    /// e.g., `true`, `null`.
    Literal(LiteralValue),

    /// The refinement applies when the function's return value is of a certain type.
    /// This handles the non-literal return case.
    Type(Type),
}

#[derive(Clone, Debug)]
pub struct CheckedBlockContents {
    pub statements: Vec<CheckedStmt>,
    pub final_expr: Option<Box<CheckedExpr>>,
}

#[derive(Clone, Debug)]
pub enum CheckedExprKind {
    Not {
        right: Box<CheckedExpr>,
    },
    Neg {
        right: Box<CheckedExpr>,
    },
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
        target: Type,
    },
    FnCall {
        left: Box<CheckedExpr>,
        args: Vec<CheckedExpr>,
    },
    StructInit {
        left: Box<CheckedExpr>,
        fields: Vec<(IdentifierNode, CheckedExpr)>,
    },
    Null,
    BoolLiteral {
        value: bool,
    },
    Number {
        value: NumberKind,
    },
    String(StringNode),
    Identifier(IdentifierNode),
    Fn {
        id: DefinitionId,
        params: Vec<CheckedParam>,
        body: CheckedBlockContents,
        return_type: Type,
        generic_params: Vec<CheckedGenericParam>,
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
    pub ty: Type,
}
