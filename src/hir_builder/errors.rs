use std::collections::HashSet;

use crate::{
    ast::{IdentifierNode, Span},
    compile::string_interner::InternerId,
    hir_builder::types::{checked_declaration::CheckedGenericParam, checked_type::Type},
    tokenize::NumberKind,
};

#[derive(Debug, Clone)]
pub enum SemanticError {
    ExpectedTypeArguments {
        span: Span,
    },
    DuplicateIdentifier {
        id: IdentifierNode,
    },
    AmbiguousGenericInferenceForUnion {
        expected: HashSet<Type>,
        received: Type,
    },
    FailedToInferGenericsInUnion {
        expected_union: HashSet<Type>,
        received: Type,
    },
    IncompatibleGenericParamSubstitution {
        generic_param: CheckedGenericParam,
        arg_type: Type,
        is_inferred: bool,
    },
    VarDeclWithoutInitializer {
        span: Span,
    },
    DuplicateStructFieldInitializer {
        id: IdentifierNode,
    },
    UnknownStructFieldInitializer {
        id: IdentifierNode,
    },
    MissingStructFieldInitializer {
        missing_fields: HashSet<InternerId>,
        span: Span,
    },
    CannotApplyStructInitializer {
        span: Span,
    },
    ExpectedANumericOperand {
        span: Span,
    },
    MixedSignedAndUnsigned {
        span: Span,
    },
    MixedFloatAndInteger {
        span: Span,
    },
    CannotCompareType {
        of: Type,
        to: Type,
    },
    UndeclaredIdentifier {
        id: IdentifierNode,
    },
    UndeclaredType {
        id: IdentifierNode,
    },
    ReturnKeywordOutsideFunction {
        span: Span,
    },
    BreakKeywordOutsideLoop {
        span: Span,
    },
    ContinueKeywordOutsideLoop {
        span: Span,
    },
    InvalidAssignmentTarget {
        target: Type,
    },
    TypeMismatch {
        expected: Type,
        received: Type,
    },
    InvalidArraySizeValue {
        value: NumberKind,
        span: Span,
    },
    ReturnNotLastStatement {
        span: Span,
    },
    ReturnTypeMismatch {
        expected: Type,
        received: Type,
    },
    CannotAccess {
        target: Type,
    },
    CannotCall {
        target: Type,
    },
    FnArgumentCountMismatch {
        expected: usize,
        received: usize,
        span: Span,
    },
    GenericArgumentCountMismatch {
        expected: usize,
        received: usize,
        span: Span,
    },
    CannotUseGenericParameterAsValue {
        span: Span,
    },
    CannotUseVariableDeclarationAsType {
        span: Span,
    },
    AccessToUndefinedField {
        field: IdentifierNode,
    },
    UnresolvedGenericParam {
        param: IdentifierNode,
        span: Span,
    },
    ConflictingGenericBinding {
        generic_param: CheckedGenericParam,
        existing: Type,
        new: Type,
    },
    TypeAliasMustBeDeclaredAtTopLevel {
        span: Span,
    },
    StructMustBeDeclaredAtTopLevel {
        span: Span,
    },
    CannotApplyTypeArguments {
        to: Type,
    },
}

impl SemanticError {
    pub fn span(&self) -> Span {
        match self {
            SemanticError::AmbiguousGenericInferenceForUnion { received, .. } => received.span,
            SemanticError::FailedToInferGenericsInUnion { received, .. } => received.span,
            SemanticError::IncompatibleGenericParamSubstitution { arg_type: with_type, .. } => with_type.span,
            SemanticError::VarDeclWithoutInitializer { span } => *span,
            SemanticError::DuplicateStructFieldInitializer { id } => id.span,
            SemanticError::UnknownStructFieldInitializer { id } => id.span,
            SemanticError::MissingStructFieldInitializer { span, .. } => *span,
            SemanticError::CannotApplyStructInitializer { span } => *span,
            SemanticError::ExpectedANumericOperand { span } => *span,
            SemanticError::MixedSignedAndUnsigned { span } => *span,
            SemanticError::MixedFloatAndInteger { span } => *span,
            SemanticError::CannotCompareType { to, .. } => to.span,
            SemanticError::UndeclaredIdentifier { id } => id.span,
            SemanticError::UndeclaredType { id } => id.span,
            SemanticError::ReturnKeywordOutsideFunction { span } => *span,
            SemanticError::BreakKeywordOutsideLoop { span } => *span,
            SemanticError::ContinueKeywordOutsideLoop { span } => *span,
            SemanticError::InvalidAssignmentTarget { target } => target.span,
            SemanticError::TypeMismatch { received, .. } => received.span,
            SemanticError::InvalidArraySizeValue { span, .. } => *span,
            SemanticError::ReturnNotLastStatement { span } => *span,
            SemanticError::ReturnTypeMismatch { received, .. } => received.span,
            SemanticError::CannotAccess { target } => target.span,
            SemanticError::CannotCall { target } => target.span,
            SemanticError::FnArgumentCountMismatch { span, .. } => *span,
            SemanticError::GenericArgumentCountMismatch { span, .. } => *span,
            SemanticError::CannotUseGenericParameterAsValue { span, .. } => *span,
            SemanticError::CannotUseVariableDeclarationAsType { span, .. } => *span,
            SemanticError::AccessToUndefinedField { field } => field.span,
            SemanticError::UnresolvedGenericParam { span, .. } => *span,
            SemanticError::ConflictingGenericBinding { new, .. } => new.span,
            SemanticError::TypeAliasMustBeDeclaredAtTopLevel { span } => *span,
            SemanticError::StructMustBeDeclaredAtTopLevel { span } => *span,
            SemanticError::CannotApplyTypeArguments { to } => to.span,
            SemanticError::DuplicateIdentifier { id } => id.span,
            SemanticError::ExpectedTypeArguments { span } => *span,
        }
    }

    pub fn code(&self) -> usize {
        match self {
            SemanticError::ExpectedANumericOperand { .. } => 1,
            SemanticError::MixedSignedAndUnsigned { .. } => 2,
            SemanticError::MixedFloatAndInteger { .. } => 3,
            SemanticError::CannotCompareType { .. } => 4,
            SemanticError::UndeclaredIdentifier { .. } => 5,
            SemanticError::ReturnKeywordOutsideFunction { .. } => 6,
            SemanticError::BreakKeywordOutsideLoop { .. } => 7,
            SemanticError::ContinueKeywordOutsideLoop { .. } => 8,
            SemanticError::InvalidAssignmentTarget { .. } => 9,
            SemanticError::TypeMismatch { .. } => 10,
            SemanticError::ReturnNotLastStatement { .. } => 11,
            SemanticError::ReturnTypeMismatch { .. } => 12,
            SemanticError::UndeclaredType { .. } => 13,
            SemanticError::CannotAccess { .. } => 14,
            SemanticError::CannotCall { .. } => 15,
            SemanticError::CannotUseGenericParameterAsValue { .. } => 16,
            SemanticError::CannotUseVariableDeclarationAsType { .. } => 17,
            SemanticError::VarDeclWithoutInitializer { .. } => 18,
            SemanticError::AccessToUndefinedField { .. } => 19,
            SemanticError::InvalidArraySizeValue { .. } => 21,
            SemanticError::FnArgumentCountMismatch { .. } => 22,
            SemanticError::GenericArgumentCountMismatch { .. } => 23,
            SemanticError::UnresolvedGenericParam { .. } => 24,
            SemanticError::ConflictingGenericBinding { .. } => 25,
            SemanticError::CannotApplyTypeArguments { .. } => 26,
            SemanticError::TypeAliasMustBeDeclaredAtTopLevel { .. } => 27,
            SemanticError::StructMustBeDeclaredAtTopLevel { .. } => 28,
            SemanticError::DuplicateStructFieldInitializer { .. } => 29,
            SemanticError::UnknownStructFieldInitializer { .. } => 30,
            SemanticError::MissingStructFieldInitializer { .. } => 31,
            SemanticError::CannotApplyStructInitializer { .. } => 32,
            SemanticError::IncompatibleGenericParamSubstitution { .. } => 33,
            SemanticError::AmbiguousGenericInferenceForUnion { .. } => 34,
            SemanticError::FailedToInferGenericsInUnion { .. } => 35,
            SemanticError::DuplicateIdentifier { .. } => 36,
            SemanticError::ExpectedTypeArguments { .. } => 37,
        }
    }
}
