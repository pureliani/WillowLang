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
    check::{check_expr::check_expr, scope::Scope, SemanticError},
};

pub fn check_inequality_expr(
    left: Box<Expr>,
    right: Box<Expr>,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let mut expr_type = CheckedType {
        kind: CheckedTypeKind::Bool,
        span: TypeSpan::Expr(Span {
            start: left.span.start,
            end: right.span.end,
        }),
    };

    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_right = check_expr(*right, errors, scope);

    // TODO: allow inequality checks for primitives

    CheckedExpr {
        kind: CheckedExprKind::NotEqual {
            left: Box::new(checked_left),
            right: Box::new(checked_right),
        },
        expr_type,
    }
}
