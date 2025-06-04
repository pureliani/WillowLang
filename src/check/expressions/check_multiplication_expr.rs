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
    compile::SpanRegistry,
};
impl<'a> SemanticChecker<'a> {}

pub fn check_multiplication_expr(
    left: Box<Expr>,
    right: Box<Expr>,
    span: Span,
    scope: Rc<RefCell<Scope>>,
    
) -> CheckedExpr {
    let checked_left = check_expr(*left, errors, scope.clone(), span_registry);
    let checked_right = check_expr(*right, errors, scope, span_registry);
    let expr_type = check_binary_numeric_operation(&checked_left, &checked_right, errors);

    CheckedExpr {
        span,
        ty: expr_type,
        kind: CheckedExprKind::Multiply {
            left: Box::new(checked_left),
            right: Box::new(checked_right),
        },
    }
}
