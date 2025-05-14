use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedTypeX, CheckedType, TypeSpan},
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
    let mut expr_type = CheckedTypeX {
        kind: CheckedType::Bool,
        span: TypeSpan::Expr(Span {
            start: left.span.start,
            end: right.span.end,
        }),
    };

    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_right = check_expr(*right, errors, scope);

    if checked_left.ty.kind != CheckedType::Bool {
        errors.push(SemanticError::new(
            SemanticErrorKind::TypeMismatch {
                expected: CheckedTypeX {
                    kind: CheckedType::Bool,
                    span: checked_left.ty.span,
                },
                received: checked_left.ty.clone(),
            },
            checked_left.ty.unwrap_expr_span(),
        ));
        expr_type.kind = CheckedType::Unknown;
    }

    if checked_right.ty.kind != CheckedType::Bool {
        errors.push(SemanticError::new(
            SemanticErrorKind::TypeMismatch {
                expected: CheckedTypeX {
                    kind: CheckedType::Bool,
                    span: checked_right.ty.span,
                },
                received: checked_right.ty.clone(),
            },
            checked_right.ty.unwrap_expr_span(),
        ));
        expr_type.kind = CheckedType::Unknown;
    }

    CheckedExpr {
        kind: CheckedExprKind::Or {
            left: Box::new(checked_left),
            right: Box::new(checked_right),
        },
        ty: expr_type,
    }
}
