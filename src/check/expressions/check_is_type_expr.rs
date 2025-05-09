use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::{base_expression::Expr, base_type::TypeAnnotation},
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
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
    expr_span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_target = check_type(&target, errors, scope);

    if !matches!(checked_left.expr_type.kind, CheckedTypeKind::Union { .. }) {
        errors.push(SemanticError::new(
            SemanticErrorKind::CannotUseIsTypeOnNonUnion,
            expr_span,
        ));
    }

    CheckedExpr {
        kind: CheckedExprKind::IsType {
            left: Box::new(checked_left),
            target: checked_target,
        },
        expr_type: CheckedType {
            kind: CheckedTypeKind::Bool,
            span: TypeSpan::Expr(expr_span),
        },
    }
}
