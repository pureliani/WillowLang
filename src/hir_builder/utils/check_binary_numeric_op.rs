use crate::{
    ast::Span,
    hir_builder::{
        errors::{SemanticError, SemanticErrorKind},
        types::checked_type::Type,
        utils::{get_numeric_type_rank::get_numeric_type_rank, is_float::is_float, is_integer::is_integer, is_signed::is_signed},
        FunctionBuilder,
    },
};

impl FunctionBuilder {
    pub fn check_binary_numeric_operation(&mut self, left: &Type, right: &Type) -> Result<Type, SemanticError> {
        let span = Span {
            start: left.span.start,
            end: right.span.end,
        };

        let left_type = if is_float(&left.kind) || is_integer(&left.kind) {
            left
        } else {
            return Err(SemanticError {
                kind: SemanticErrorKind::ExpectedANumericOperand,
                span: left.span,
            });
        };

        let right_type = if is_float(&right.kind) || is_integer(&right.kind) {
            right
        } else {
            return Err(SemanticError {
                kind: SemanticErrorKind::ExpectedANumericOperand,
                span: right.span,
            });
        };

        if (is_float(&left_type.kind) && is_integer(&right_type.kind))
            || (is_integer(&left_type.kind) && is_float(&right_type.kind))
        {
            return Err(SemanticError {
                kind: SemanticErrorKind::MixedFloatAndInteger,
                span,
            });
        }

        if is_signed(&left_type.kind) != is_signed(&right_type.kind) {
            return Err(SemanticError {
                kind: SemanticErrorKind::MixedSignedAndUnsigned,
                span,
            });
        }

        if right_type == left_type {
            return Ok(left_type.clone());
        }

        let left_rank = get_numeric_type_rank(&left_type.kind);
        let right_rank = get_numeric_type_rank(&right_type.kind);

        if left_rank > right_rank {
            Ok(Type {
                kind: left_type.kind.clone(),
                span,
            })
        } else {
            Ok(Type {
                kind: right_type.kind.clone(),
                span,
            })
        }
    }
}
