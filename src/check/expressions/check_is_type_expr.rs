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
    compile::SpanRegistry,
};
impl<'a> SemanticChecker<'a> {}

pub fn check_is_type_expr(
    left: Box<Expr>,
    target: TypeAnnotation,
    span: Span,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_left = check_expr(*left, errors, scope.clone(), span_registry);
    let checked_target = check_type(&target, errors, scope, span_registry);

    // TODO: do an actual check
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
