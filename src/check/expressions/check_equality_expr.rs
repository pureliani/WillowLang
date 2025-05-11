use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
        },
        Span,
    },
    check::{
        check_expr::check_expr,
        scope::Scope,
        utils::{get_numeric_type_rank::get_numeric_type_rank, is_integer::is_integer},
        SemanticError, SemanticErrorKind,
    },
};

pub fn check_equality_expr(
    left: Box<Expr>,
    right: Box<Expr>,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let span = Span {
        start: left.span.start,
        end: right.span.end,
    };
    let mut expr_type = CheckedType {
        kind: CheckedTypeKind::Bool,
        span: TypeSpan::Expr(span.clone()),
    };

    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_right = check_expr(*right, errors, scope);

    let err = SemanticError::new(
        SemanticErrorKind::CannotCompareType {
            of: checked_left.expr_type.clone(),
            to: checked_right.expr_type.clone(),
        },
        span,
    );

    if !is_integer(&checked_left.expr_type)
        || !is_integer(&checked_right.expr_type)
        || get_numeric_type_rank(&checked_left.expr_type)
            < get_numeric_type_rank(&checked_right.expr_type)
    {
        errors.push(err);
        expr_type.kind = CheckedTypeKind::Unknown
    } else {
        match (&checked_left.expr_type.kind, &checked_right.expr_type.kind) {
            (CheckedTypeKind::Bool, CheckedTypeKind::Bool) => {}
            (CheckedTypeKind::Char, CheckedTypeKind::Char) => {}
            (CheckedTypeKind::Null, CheckedTypeKind::Null) => {}
            (CheckedTypeKind::Enum(_), CheckedTypeKind::Enum(_)) => {}
            _ => {
                errors.push(err);
                expr_type.kind = CheckedTypeKind::Unknown
            }
        }
    }

    CheckedExpr {
        kind: CheckedExprKind::Equal {
            left: Box::new(checked_left),
            right: Box::new(checked_right),
        },
        expr_type,
    }
}
