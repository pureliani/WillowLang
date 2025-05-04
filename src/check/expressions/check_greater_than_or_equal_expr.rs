use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{Type, TypeKind},
        },
    },
    check::{
        check_expr::check_expr, scope::Scope,
        utils::check_binary_numeric_operation::check_binary_numeric_operation, SemanticError,
    },
};

pub fn check_greater_than_or_equal_expr(
    left: Box<Expr>,
    right: Box<Expr>,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_right = check_expr(*right, errors, scope);
    let checked_op = check_binary_numeric_operation(&checked_left, &checked_right, errors);

    let type_kind = if checked_op.kind == TypeKind::Unknown {
        TypeKind::Unknown
    } else {
        TypeKind::Bool
    };

    let expr_type = Type {
        kind: type_kind,
        span: checked_op.span,
    };

    CheckedExpr {
        kind: CheckedExprKind::GreaterThanOrEqual {
            left: Box::new(checked_left),
            right: Box::new(checked_right),
        },
        expr_type,
    }
}
