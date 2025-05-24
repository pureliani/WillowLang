use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::CheckedType,
        },
        Span,
    },
    check::{
        check_expr::check_expr, scope::Scope, utils::check_is_assignable::check_is_assignable,
        SemanticError, SemanticErrorKind,
    },
};

pub fn check_and_expr(
    left: Box<Expr>,
    right: Box<Expr>,
    span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let mut expr_type = CheckedType::Bool;

    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_right = check_expr(*right, errors, scope);

    if !check_is_assignable(&checked_left.ty, &CheckedType::Bool) {
        errors.push(SemanticError {
            kind: SemanticErrorKind::TypeMismatch {
                expected: CheckedType::Bool,
                received: checked_left.ty.clone(),
            },
            span: checked_left.span,
        });

        expr_type = CheckedType::Unknown;
    }

    if !check_is_assignable(&checked_right.ty, &CheckedType::Bool) {
        errors.push(SemanticError {
            kind: SemanticErrorKind::TypeMismatch {
                expected: CheckedType::Bool,
                received: checked_right.ty.clone(),
            },
            span: checked_right.span,
        });
        expr_type = CheckedType::Unknown;
    }

    CheckedExpr {
        kind: CheckedExprKind::And {
            left: Box::new(checked_left),
            right: Box::new(checked_right),
        },
        span,
        ty: expr_type,
    }
}
