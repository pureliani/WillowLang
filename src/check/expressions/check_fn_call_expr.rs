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

pub fn check_fn_call_expr(
    left: Box<Expr>,
    args: Vec<Expr>,
    span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_args: Vec<_> = args
        .into_iter()
        .map(|arg| check_expr(arg, errors, scope.clone()))
        .collect();

    let mut call_result_type = CheckedType::Unknown;

    match &checked_left.ty {
        CheckedType::FnType {
            params,
            return_type,
        } => {
            call_result_type = *return_type.clone();

            if checked_args.len() != params.len() {
                errors.push(SemanticError::new(
                    SemanticErrorKind::ArgumentCountMismatch {
                        expected: params.len(),
                        received: checked_args.len(),
                    },
                    span,
                ));
            } else {
                for (param, arg) in params.iter().zip(checked_args.iter()) {
                    if !check_is_assignable(&arg.ty, &param.constraint) {
                        errors.push(SemanticError::new(
                            SemanticErrorKind::TypeMismatch {
                                expected: param.constraint.clone(),
                                received: arg.ty.clone(),
                            },
                            arg.span,
                        ));
                    }
                }
            }
        }
        CheckedType::GenericFnType {
            params,
            return_type,
            generic_params,
        } => {
            // --- Call on a Generic Function without Explicit Type Arguments ---
            // This requires type inference, which is complex.
            // For now, let's require explicit arguments for generic functions.
            todo!("Implement type inference and substitution")
        }
        non_callable_type => {
            errors.push(SemanticError::new(
                SemanticErrorKind::CannotCall(checked_left.ty.clone()),
                checked_left.span,
            ));
        }
    }

    CheckedExpr {
        span,
        ty: call_result_type,
        kind: CheckedExprKind::FnCall {
            left: Box::new(checked_left),
            args: checked_args,
        },
    }
}
