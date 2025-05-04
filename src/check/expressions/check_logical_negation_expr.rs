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

pub fn check_logical_negation_expr(
    right: Box<Expr>,
    span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let right_span = right.span;
    let checked_right = check_expr(*right, errors, scope);

    let mut expr_type = Type {
        kind: TypeKind::Bool,
        span: TypeSpan::Expr(span),
    };

    if checked_right.expr_type.kind != TypeKind::Bool {
        errors.push(SemanticError::new(
            SemanticErrorKind::TypeMismatch {
                expected: Type {
                    kind: TypeKind::Bool,
                    span: TypeSpan::Expr(right_span),
                },
                received: checked_right.expr_type.clone(),
            },
            span,
        ));
        expr_type.kind = TypeKind::Unknown
    }

    CheckedExpr {
        kind: CheckedExprKind::Not {
            right: Box::new(checked_right),
        },
        expr_type,
    }
}
