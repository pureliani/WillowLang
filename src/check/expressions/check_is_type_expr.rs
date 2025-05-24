use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::{base_expression::Expr, base_type::TypeAnnotation},
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::CheckedType,
        },
        Span,
    },
    check::{
        check_expr::check_expr, scope::Scope, utils::type_annotation_to_semantic::check_type,
        SemanticError, SemanticErrorKind,
    },
};

pub fn check_is_type_expr(
    left: Box<Expr>,
    target: TypeAnnotation,
    span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_target = check_type(&target, errors, scope);

    if !matches!(checked_left.ty, CheckedType::Union { .. }) {
        errors.push(SemanticError {
            kind: SemanticErrorKind::CannotUseIsTypeOnNonUnion,
            span,
        });
    }

    CheckedExpr {
        span,
        ty: CheckedType::Bool,
        kind: CheckedExprKind::IsType {
            left: Box::new(checked_left),
            target: checked_target,
        },
    }
}
