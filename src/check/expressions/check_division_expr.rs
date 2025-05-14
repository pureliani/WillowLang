use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::checked_expression::{CheckedExpr, CheckedExprKind},
        Span,
    },
    check::{
        check_expr::check_expr, scope::Scope,
        utils::check_binary_numeric_operation::check_binary_numeric_operation, SemanticError,
    },
};

pub fn check_division_expr(
    left: Box<Expr>,
    right: Box<Expr>,
    span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_right = check_expr(*right, errors, scope);
    let expr_type = check_binary_numeric_operation(&checked_left, &checked_right, errors);

    CheckedExpr {
        span,
        kind: CheckedExprKind::Divide {
            left: Box::new(checked_left),
            right: Box::new(checked_right),
        },
        ty: expr_type,
    }
}
