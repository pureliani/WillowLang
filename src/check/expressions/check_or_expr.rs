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
    compile::SpanRegistry,
};
impl<'a> SemanticChecker<'a> {}

pub fn check_or_expr(
    left: Box<Expr>,
    right: Box<Expr>,
    span: Span,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let mut ty = CheckedType::Bool;

    let checked_left = check_expr(*left, errors, scope.clone(), span_registry);
    let checked_right = check_expr(*right, errors, scope, span_registry);

    if !check_is_assignable(&checked_left.ty, &CheckedType::Bool) {
        errors.push(SemanticError {
            kind: SemanticErrorKind::TypeMismatch {
                expected: CheckedType::Bool,
                received: checked_left.ty.clone(),
            },
            span: checked_left.span,
        });
        ty = CheckedType::Unknown;
    }

    if !check_is_assignable(&checked_right.ty, &CheckedType::Bool) {
        errors.push(SemanticError {
            kind: SemanticErrorKind::TypeMismatch {
                expected: CheckedType::Bool,
                received: checked_right.ty.clone(),
            },
            span: checked_right.span,
        });
        ty = CheckedType::Unknown;
    }

    CheckedExpr {
        span,
        kind: CheckedExprKind::Or {
            left: Box::new(checked_left),
            right: Box::new(checked_right),
        },
        ty,
    }
}
