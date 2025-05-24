use std::{cell::RefCell, collections::HashSet, rc::Rc};

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

    let expr_type = match &checked_right.ty {
        t if is_signed(&t) => t.clone(),
        unexpected_type => {
            let expected = HashSet::from([
                CheckedType::I8,
                CheckedType::I16,
                CheckedType::I32,
                CheckedType::I64,
                CheckedType::ISize,
                CheckedType::F32,
                CheckedType::F64,
            ]);

            errors.push(SemanticError {
                kind: SemanticErrorKind::TypeMismatch {
                    expected: CheckedType::Union(expected),
                    received: unexpected_type.clone(),
                },
                span: checked_right.span,
            });

            CheckedType::Unknown
        }
    };

    CheckedExpr {
        span,
        ty: expr_type,
        kind: CheckedExprKind::Neg {
            right: Box::new(checked_right),
        },
    }
}
