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
        check_expr::check_expr,
        scope::Scope,
        utils::{get_numeric_type_rank::get_numeric_type_rank, is_integer::is_integer},
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

    let err = SemanticError::new(
        SemanticErrorKind::CannotCompareType {
            of: checked_left.ty.clone(),
            to: checked_right.ty.clone(),
        },
        span,
    );

    if !is_integer(&checked_left.ty)
        || !is_integer(&checked_right.ty)
        || get_numeric_type_rank(&checked_left.ty) < get_numeric_type_rank(&checked_right.ty)
    {
        errors.push(err);
        ty = CheckedType::Unknown
    } else {
        match (&checked_left.ty, &checked_right.ty) {
            (CheckedType::Bool, CheckedType::Bool) => {}
            (CheckedType::Char, CheckedType::Char) => {}
            (CheckedType::Null, CheckedType::Null) => {}
            (CheckedType::Enum(_), CheckedType::Enum(_)) => {}
            _ => {
                errors.push(err);
                ty = CheckedType::Unknown
            }
        }
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
