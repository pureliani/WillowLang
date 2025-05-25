use crate::{
    ast::{
        checked::{checked_expression::CheckedExpr, checked_type::CheckedType},
        Span,
    },
    check::{SemanticError, SemanticErrorKind},
};

use super::{
    get_numeric_type_rank::get_numeric_type_rank, is_float::is_float, is_integer::is_integer,
    is_signed::is_signed,
};

pub fn check_binary_numeric_operation(
    left: &CheckedExpr,
    right: &CheckedExpr,
    errors: &mut Vec<SemanticError>,
) -> CheckedType {
    let combined_span = Span {
        start: left.span.start,
        end: right.span.end,
    };

    let left_type = if is_float(&left.ty) || is_integer(&left.ty) {
        &left.ty
    } else {
        errors.push(SemanticError {
            kind: SemanticErrorKind::ExpectedANumericOperand,
            span: left.span,
        });

        return CheckedType::Unknown;
    };

    let right_type = if is_float(&right.ty) || is_integer(&right.ty) {
        &right.ty
    } else {
        errors.push(SemanticError {
            kind: SemanticErrorKind::ExpectedANumericOperand,
            span: right.span,
        });

        return CheckedType::Unknown;
    };

    if (is_float(&left_type) && is_integer(&right_type))
        || (is_integer(&left_type) && is_float(&right_type))
    {
        errors.push(SemanticError {
            kind: SemanticErrorKind::MixedFloatAndInteger,
            span: combined_span,
        });

        return CheckedType::Unknown;
    }

    if is_signed(&left_type) != is_signed(&right_type) {
        errors.push(SemanticError {
            kind: SemanticErrorKind::MixedSignedAndUnsigned,
            span: combined_span,
        });

        return CheckedType::Unknown;
    }

    if right_type == left_type {
        return left_type.clone();
    }

    let left_rank = get_numeric_type_rank(&left_type);
    let right_rank = get_numeric_type_rank(&right_type);

    if left_rank > right_rank {
        left_type.clone()
    } else {
        right_type.clone()
    }
}
