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
    compile::SpanRegistry,
};
impl<'a> SemanticChecker<'a> {}

pub fn check_inequality_expr(
    left: Box<Expr>,
    right: Box<Expr>,
    span: Span,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let mut ty = CheckedType::Bool;

    let checked_left = check_expr(*left, errors, scope.clone(), span_registry);
    let checked_right = check_expr(*right, errors, scope, span_registry);

    if !check_is_equatable(&checked_left.ty, &checked_right.ty) {
        errors.push(SemanticError {
            kind: SemanticErrorKind::CannotCompareType {
                of: checked_left.ty.clone(),
                to: checked_right.ty.clone(),
            },
            span,
        });

        ty = CheckedType::Unknown;
    }

    CheckedExpr {
        ty,
        span,
        kind: CheckedExprKind::NotEqual {
            left: Box::new(checked_left),
            right: Box::new(checked_right),
        },
    }
}
