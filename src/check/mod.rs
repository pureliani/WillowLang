use crate::{
    ast::{checked::checked_type::Type, IdentifierNode, Span},
    tokenizer::NumberKind,
};

pub mod check_expr;
pub mod check_is_assignable;
pub mod check_returns;
pub mod check_stmt;
pub mod check_stmts;
pub mod scope;
pub mod type_annotation_to_semantic;
pub mod type_flow_graph;

#[derive(Debug, Clone)]
pub enum SemanticErrorKind {
    NonNumericOperand,
    MixedSignedAndUnsigned,
    MixedFloatAndInteger,
    CannotCompareType { of: Type, to: Type },
    UndeclaredIdentifier(String),
    UndeclaredType(String),
    ReturnKeywordOutsideFunction,
    BreakKeywordOutsideLoop,
    ContinueKeywordOutsideLoop,
    InvalidAssignmentTarget,
    TypeMismatch { expected: Type, received: Type },
    InvalidArraySizeValue(NumberKind),
    ReturnNotLastStatement,
    ReturnTypeMismatch { expected: Type, received: Type },
    CannotAccess(Type),
    CannotCall(Type),
    ArgumentCountMismatch { expected: usize, received: usize },
    GenericArgumentCountMismatch { expected: usize, received: usize },
    CannotUseGenericParameterAsValue,
    CannotUseVariableDeclarationAsType,
    VarDeclWithNoConstraintOrInitializer,
    UndefinedProperty(IdentifierNode),
    UnresolvedGenericParam(String),
    CannotUseIsTypeOnNonUnion,
    ConflictingGenericBinding { existing: Type, new: Type },
}

#[derive(Debug, Clone)]
pub struct SemanticError {
    kind: SemanticErrorKind,
    code: usize,
    span: Span,
}

impl SemanticError {
    fn kind_to_code(kind: &SemanticErrorKind) -> usize {
        match kind {
            SemanticErrorKind::NonNumericOperand => 1,
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
            SemanticErrorKind::UndefinedProperty(..) => 19,
            SemanticErrorKind::CannotUseIsTypeOnNonUnion => 20,
            SemanticErrorKind::InvalidArraySizeValue(..) => 21,
            SemanticErrorKind::ArgumentCountMismatch { .. } => 22,
            SemanticErrorKind::GenericArgumentCountMismatch { .. } => 23,
            SemanticErrorKind::UnresolvedGenericParam(..) => 24,
            SemanticErrorKind::ConflictingGenericBinding { .. } => 25,
        }
    }

    fn new(kind: SemanticErrorKind, span: Span) -> Self {
        let code = Self::kind_to_code(&kind);

        Self { code, kind, span }
    }

    fn get_kind(&self) -> &SemanticErrorKind {
        &self.kind
    }
}
