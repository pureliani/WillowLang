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
        check_expr::check_expr, scope::Scope,
        utils::check_binary_numeric_operation::check_binary_numeric_operation, SemanticError,
    },
};

pub fn check_less_than_or_equal_expr(
    left: Box<Expr>,
    right: Box<Expr>,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let span = Span {
        start: left.span.start,
        end: right.span.end,
    };
    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_right = check_expr(*right, errors, scope);
    let checked_op = check_binary_numeric_operation(&checked_left, &checked_right, errors);

    let expr_type = if checked_op == CheckedType::Unknown {
        CheckedType::Unknown
    } else {
        CheckedType::Bool
    };

    CheckedExpr {
        span,
        ty: expr_type,
        kind: CheckedExprKind::LessThanOrEqual {
            left: Box::new(checked_left),
            right: Box::new(checked_right),
        },
    }
}
