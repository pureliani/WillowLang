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
        check_expr::check_expr, scope::Scope, utils::check_is_equatable::check_is_equatable,
        SemanticError, SemanticErrorKind,
    },
};

pub fn check_equality_expr(
    left: Box<Expr>,
    right: Box<Expr>,
    span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let mut ty = CheckedType::Bool;

    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_right = check_expr(*right, errors, scope);

    if !check_is_equatable(&checked_left.ty, &checked_right.ty) {
        errors.push(SemanticError {
            kind: SemanticErrorKind::CannotCompareType {
                of: checked_left.ty.clone(),
                to: checked_right.ty.clone(),
            },
            span,
        });

        ty = CheckedType::Unknown
    }

    CheckedExpr {
        ty,
        span,
        kind: CheckedExprKind::Equal {
            left: Box::new(checked_left),
            right: Box::new(checked_right),
        },
    }
}
