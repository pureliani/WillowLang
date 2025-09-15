use std::collections::HashSet;

use crate::{
    ast::{IdentifierNode, Span},
    compile::string_interner::InternerId,
    hir_builder::types::checked_type::Type,
};

#[derive(Debug, Clone)]
pub enum SemanticErrorKind {
    DuplicateIdentifier(IdentifierNode),
    VarDeclWithoutInitializer,
    DuplicateStructFieldInitializer(IdentifierNode),
    UnknownStructFieldInitializer(IdentifierNode),
    MissingStructFieldInitializers(HashSet<InternerId>),
    CannotApplyStructInitializer,
    ExpectedANumericOperand,
    IncompatibleBranchTypes { first: Type, second: Type },
    MixedSignedAndUnsigned,
    MixedFloatAndInteger,
    ExpectedUnionType,
    CannotCompareType { of: Type, to: Type },
    UndeclaredIdentifier(IdentifierNode),
    UndeclaredType(IdentifierNode),
    ReturnKeywordOutsideFunction,
    BreakKeywordOutsideLoop,
    ContinueKeywordOutsideLoop,
    InvalidLValue,
    TypeMismatch { expected: Type, received: Type },
    TypeMismatchExpectedOneOf { expected: HashSet<Type>, received: Type },
    ReturnNotLastStatement,
    ReturnTypeMismatch { expected: Type, received: Type },
    CannotAccess(Type),
    CannotCall(Type),
    FnArgumentCountMismatch { expected: usize, received: usize },
    CannotUseVariableDeclarationAsType,
    AccessToUndefinedField { field: IdentifierNode },
    TypeAliasMustBeDeclaredAtTopLevel,
    StructMustBeDeclaredAtTopLevel,
    IfExpressionMissingElse,
    CannotCastType { source_type: Type, target_type: Type },
}

#[derive(Debug, Clone)]
pub struct SemanticError {
    pub kind: SemanticErrorKind,
    pub span: Span,
}

impl SemanticErrorKind {
    pub fn code(&self) -> usize {
        match self {
            SemanticErrorKind::ExpectedANumericOperand { .. } => 1,
            SemanticErrorKind::MixedSignedAndUnsigned { .. } => 2,
            SemanticErrorKind::MixedFloatAndInteger { .. } => 3,
            SemanticErrorKind::CannotCompareType { .. } => 4,
            SemanticErrorKind::UndeclaredIdentifier { .. } => 5,
            SemanticErrorKind::ReturnKeywordOutsideFunction { .. } => 6,
            SemanticErrorKind::BreakKeywordOutsideLoop { .. } => 7,
            SemanticErrorKind::ContinueKeywordOutsideLoop { .. } => 8,
            SemanticErrorKind::InvalidLValue { .. } => 9,
            SemanticErrorKind::TypeMismatch { .. } => 10,
            SemanticErrorKind::ReturnNotLastStatement { .. } => 11,
            SemanticErrorKind::ReturnTypeMismatch { .. } => 12,
            SemanticErrorKind::UndeclaredType { .. } => 13,
            SemanticErrorKind::CannotAccess { .. } => 14,
            SemanticErrorKind::CannotCall { .. } => 15,
            SemanticErrorKind::CannotUseVariableDeclarationAsType { .. } => 16,
            SemanticErrorKind::VarDeclWithoutInitializer { .. } => 17,
            SemanticErrorKind::AccessToUndefinedField { .. } => 18,
            SemanticErrorKind::FnArgumentCountMismatch { .. } => 19,
            SemanticErrorKind::TypeAliasMustBeDeclaredAtTopLevel { .. } => 20,
            SemanticErrorKind::StructMustBeDeclaredAtTopLevel { .. } => 21,
            SemanticErrorKind::DuplicateStructFieldInitializer { .. } => 22,
            SemanticErrorKind::UnknownStructFieldInitializer { .. } => 23,
            SemanticErrorKind::MissingStructFieldInitializers { .. } => 24,
            SemanticErrorKind::CannotApplyStructInitializer { .. } => 25,
            SemanticErrorKind::DuplicateIdentifier { .. } => 26,
            SemanticErrorKind::IncompatibleBranchTypes { .. } => 27,
            SemanticErrorKind::IfExpressionMissingElse => 28,
            SemanticErrorKind::TypeMismatchExpectedOneOf { .. } => 29,
            SemanticErrorKind::ExpectedUnionType => 30,
            SemanticErrorKind::CannotCastType { .. } => 31,
        }
    }
}
