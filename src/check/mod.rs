use std::collections::HashSet;

use crate::{
    ast::{checked::checked_type::CheckedType, IdentifierNode, Span},
    compile::string_interner::InternerId,
    tokenize::NumberKind,
};

pub mod check_expr;
pub mod check_stmt;
pub mod check_stmts;
pub mod expressions;
pub mod scope;
pub mod utils;

#[derive(Debug, Clone)]
pub enum SemanticErrorKind {
    DuplicateStructPropertyInitializer(IdentifierNode),
    UnknownStructPropertyInitializer(IdentifierNode),
    MissingStructPropertyInitializer(HashSet<InternerId>),
    CannotApplyStructInitializer,
    ExpectedANumericOperand,
    MixedSignedAndUnsigned,
    MixedFloatAndInteger,
    CannotCompareType {
        of: CheckedType,
        to: CheckedType,
    },
    UndeclaredIdentifier(IdentifierNode),
    UndeclaredType(IdentifierNode),
    ReturnKeywordOutsideFunction,
    BreakKeywordOutsideLoop,
    ContinueKeywordOutsideLoop,
    InvalidAssignmentTarget,
    TypeMismatch {
        expected: CheckedType,
        received: CheckedType,
    },
    InvalidArraySizeValue(NumberKind),
    ReturnNotLastStatement,
    ReturnTypeMismatch {
        expected: CheckedType,
        received: CheckedType,
    },
    CannotAccess(CheckedType),
    CannotCall(CheckedType),
    FnArgumentCountMismatch {
        expected: usize,
        received: usize,
    },
    GenericArgumentCountMismatch {
        expected: usize,
        received: usize,
    },
    CannotUseGenericParameterAsValue,
    CannotUseVariableDeclarationAsType,
    VarDeclWithNoConstraintOrInitializer,
    AccessToUndefinedProperty(IdentifierNode),
    UnresolvedGenericParam(IdentifierNode),
    CannotUseIsTypeOnNonUnion,
    ConflictingGenericBinding {
        identifier: IdentifierNode,
        existing: CheckedType,
        new: CheckedType,
    },
    TypeAliasMustBeDeclaredAtTopLevel,
    StructMustBeDeclaredAtTopLevel,
    CannotApplyTypeArguments {
        to: CheckedType,
    },
}

impl SemanticErrorKind {
    pub fn code(&self) -> usize {
        match self {
            SemanticErrorKind::ExpectedANumericOperand => 1,
            SemanticErrorKind::MixedSignedAndUnsigned => 2,
            SemanticErrorKind::MixedFloatAndInteger => 3,
            SemanticErrorKind::CannotCompareType { .. } => 4,
            SemanticErrorKind::UndeclaredIdentifier { .. } => 5,
            SemanticErrorKind::ReturnKeywordOutsideFunction => 6,
            SemanticErrorKind::BreakKeywordOutsideLoop => 7,
            SemanticErrorKind::ContinueKeywordOutsideLoop => 8,
            SemanticErrorKind::InvalidAssignmentTarget => 9,
            SemanticErrorKind::TypeMismatch { .. } => 10,
            SemanticErrorKind::ReturnNotLastStatement => 11,
            SemanticErrorKind::ReturnTypeMismatch { .. } => 12,
            SemanticErrorKind::UndeclaredType(..) => 13,
            SemanticErrorKind::CannotAccess(..) => 14,
            SemanticErrorKind::CannotCall(..) => 15,
            SemanticErrorKind::CannotUseGenericParameterAsValue => 16,
            SemanticErrorKind::CannotUseVariableDeclarationAsType => 17,
            SemanticErrorKind::VarDeclWithNoConstraintOrInitializer => 18,
            SemanticErrorKind::AccessToUndefinedProperty(..) => 19,
            SemanticErrorKind::CannotUseIsTypeOnNonUnion => 20,
            SemanticErrorKind::InvalidArraySizeValue(..) => 21,
            SemanticErrorKind::FnArgumentCountMismatch { .. } => 22,
            SemanticErrorKind::GenericArgumentCountMismatch { .. } => 23,
            SemanticErrorKind::UnresolvedGenericParam(..) => 24,
            SemanticErrorKind::ConflictingGenericBinding { .. } => 25,
            SemanticErrorKind::CannotApplyTypeArguments { .. } => 26,
            SemanticErrorKind::TypeAliasMustBeDeclaredAtTopLevel => 27,
            SemanticErrorKind::StructMustBeDeclaredAtTopLevel => 28,
            SemanticErrorKind::DuplicateStructPropertyInitializer { .. } => 29,
            SemanticErrorKind::UnknownStructPropertyInitializer { .. } => 30,
            SemanticErrorKind::MissingStructPropertyInitializer { .. } => 31,
            SemanticErrorKind::CannotApplyStructInitializer => 32,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SemanticError {
    pub kind: SemanticErrorKind,
    pub span: Span,
}
