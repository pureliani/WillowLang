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
    CannotCompareType { of: Type, to: Type },
    UndeclaredIdentifier(IdentifierNode),
    UndeclaredType(IdentifierNode),
    ReturnKeywordOutsideFunction,
    BreakKeywordOutsideLoop,
    ContinueKeywordOutsideLoop,
    InvalidLValue,
    TypeMismatch { expected: Type, received: Type },
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
            SemanticErrorKind::CannotUseVariableDeclarationAsType { .. } => 17,
            SemanticErrorKind::VarDeclWithoutInitializer { .. } => 18,
            SemanticErrorKind::AccessToUndefinedField { .. } => 19,
            SemanticErrorKind::FnArgumentCountMismatch { .. } => 20,
            SemanticErrorKind::TypeAliasMustBeDeclaredAtTopLevel { .. } => 21,
            SemanticErrorKind::StructMustBeDeclaredAtTopLevel { .. } => 22,
            SemanticErrorKind::DuplicateStructFieldInitializer { .. } => 23,
            SemanticErrorKind::UnknownStructFieldInitializer { .. } => 24,
            SemanticErrorKind::MissingStructFieldInitializers { .. } => 25,
            SemanticErrorKind::CannotApplyStructInitializer { .. } => 26,
            SemanticErrorKind::DuplicateIdentifier { .. } => 27,
            SemanticErrorKind::IncompatibleBranchTypes { first, second } => 28,
            SemanticErrorKind::IfExpressionMissingElse => 29,
        }
    }
}
