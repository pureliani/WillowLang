use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{Type, TypeKind, TypeSpan},
        },
        Span,
    },
    check::{check_expr::check_expr, scope::Scope, SemanticError, SemanticErrorKind},
};

pub fn check_or_expr(
    left: Box<Expr>,
    right: Box<Expr>,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let mut expr_type = Type {
        kind: TypeKind::Bool,
        span: TypeSpan::Expr(Span {
            start: left.span.start,
            end: right.span.end,
        }),
    };

    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_right = check_expr(*right, errors, scope);

    if checked_left.expr_type.kind != TypeKind::Bool {
        errors.push(SemanticError::new(
            SemanticErrorKind::TypeMismatch {
                expected: Type {
                    kind: TypeKind::Bool,
                    span: checked_left.expr_type.span,
                },
                received: checked_left.expr_type.clone(),
            },
            checked_left.expr_type.unwrap_expr_span(),
        ));
        expr_type.kind = TypeKind::Unknown;
    }

    if checked_right.expr_type.kind != TypeKind::Bool {
        errors.push(SemanticError::new(
            SemanticErrorKind::TypeMismatch {
                expected: Type {
                    kind: TypeKind::Bool,
                    span: checked_right.expr_type.span,
                },
                received: checked_right.expr_type.clone(),
            },
            checked_right.expr_type.unwrap_expr_span(),
        ));
        expr_type.kind = TypeKind::Unknown;
    }

    CheckedExpr {
        kind: CheckedExprKind::Or {
            left: Box::new(checked_left),
            right: Box::new(checked_right),
        },
        expr_type,
    }
}
