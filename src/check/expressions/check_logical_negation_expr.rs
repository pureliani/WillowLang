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

pub fn check_logical_negation_expr(
    right: Box<Expr>,
    span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_right = check_expr(*right, errors, scope);

    let mut expr_type = CheckedType::Bool;

    if !check_is_assignable(&checked_right.ty, &CheckedType::Bool) {
        errors.push(SemanticError {
            kind: SemanticErrorKind::TypeMismatch {
                expected: CheckedType::Bool,
                received: checked_right.ty.clone(),
            },
            span,
        });
        expr_type = CheckedType::Unknown
    }

    CheckedExpr {
        span,
        ty: expr_type,
        kind: CheckedExprKind::Not {
            right: Box::new(checked_right),
        },
    }
}
