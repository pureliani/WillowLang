use crate::{
    ast::{
        checked::{
            checked_expression::CheckedExpr,
            checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
        },
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
        start: left.expr_type.unwrap_expr_span().start,
        end: right.expr_type.unwrap_expr_span().start,
    };

    let left_type = if is_float(&left.expr_type) || is_integer(&left.expr_type) {
        &left.expr_type
    } else {
        errors.push(SemanticError::new(
            SemanticErrorKind::NonNumericOperand,
            left.expr_type.unwrap_expr_span(),
        ));

        return CheckedType {
            kind: CheckedTypeKind::Unknown,
            span: TypeSpan::Expr(combined_span),
        };
    };

    let right_type = if is_float(&right.expr_type) || is_integer(&right.expr_type) {
        &right.expr_type
    } else {
        errors.push(SemanticError::new(
            SemanticErrorKind::NonNumericOperand,
            right.expr_type.unwrap_expr_span(),
        ));

        return CheckedType {
            kind: CheckedTypeKind::Unknown,
            span: TypeSpan::Expr(combined_span),
        };
    };

    if (is_float(&left_type) && is_integer(&right_type))
        || (is_integer(&left_type) && is_float(&right_type))
    {
        errors.push(SemanticError::new(
            SemanticErrorKind::MixedFloatAndInteger,
            combined_span,
        ));
        return CheckedType {
            kind: CheckedTypeKind::Unknown,
            span: TypeSpan::Expr(combined_span),
        };
    }

    if is_signed(&left_type) != is_signed(&right_type) {
        errors.push(SemanticError::new(
            SemanticErrorKind::MixedSignedAndUnsigned,
            combined_span,
        ));
        return CheckedType {
            kind: CheckedTypeKind::Unknown,
            span: TypeSpan::Expr(combined_span),
        };
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
