use std::{cell::RefCell, collections::HashSet, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
        },
        Span,
    },
    check::{
        check_expr::check_expr, scope::Scope, utils::is_signed::is_signed, SemanticError,
        SemanticErrorKind,
    },
};

pub fn check_arithmetic_negation_expr(
    right: Box<Expr>,
    span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_right = check_expr(*right, errors, scope);

    let expr_type = match &checked_right.expr_type {
        t if is_signed(&t) => t.clone(),
        unexpected_type => {
            let expected = HashSet::from([
                CheckedType {
                    kind: CheckedTypeKind::I8,
                    span: checked_right.expr_type.span,
                },
                CheckedType {
                    kind: CheckedTypeKind::I16,
                    span: checked_right.expr_type.span,
                },
                CheckedType {
                    kind: CheckedTypeKind::I32,
                    span: checked_right.expr_type.span,
                },
                CheckedType {
                    kind: CheckedTypeKind::I64,
                    span: checked_right.expr_type.span,
                },
                CheckedType {
                    kind: CheckedTypeKind::ISize,
                    span: checked_right.expr_type.span,
                },
                CheckedType {
                    kind: CheckedTypeKind::F32,
                    span: checked_right.expr_type.span,
                },
                CheckedType {
                    kind: CheckedTypeKind::F64,
                    span: checked_right.expr_type.span,
                },
            ]);

            errors.push(SemanticError::new(
                SemanticErrorKind::TypeMismatch {
                    expected: CheckedType {
                        kind: CheckedTypeKind::Union(expected),
                        span: checked_right.expr_type.span,
                    },
                    received: unexpected_type.clone(),
                },
                checked_right.expr_type.unwrap_expr_span(),
            ));

            CheckedType {
                kind: CheckedTypeKind::Unknown,
                span: TypeSpan::Expr(span),
            }
        }
    };

    CheckedExpr {
        expr_type,
        kind: CheckedExprKind::Neg {
            right: Box::new(checked_right),
        },
    }
}
