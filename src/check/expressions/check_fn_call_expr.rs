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
    check::{
        check_expr::check_expr,
        scope::Scope,
        utils::{
            check_is_assignable::check_is_assignable,
            substitute_generics::{substitute_generics, GenericSubstitutionMap},
        },
        SemanticError, SemanticErrorKind,
    },
};

pub fn check_fn_call_expr(
    left: Box<Expr>,
    args: Vec<Expr>,
    expr_span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_args: Vec<_> = args
        .into_iter()
        .map(|arg| check_expr(arg, errors, scope.clone()))
        .collect();

    let mut call_result_type = Type {
        kind: TypeKind::Unknown,
        span: TypeSpan::Expr(expr_span),
    };

    match &checked_left.expr_type.kind {
        TypeKind::GenericFnType {
            generic_params,
            params,
            return_type,
        } if !generic_params.is_empty() => {
            // --- Call on a Generic Function without Explicit Type Arguments ---
            // This requires type inference, which is complex.
            // For now, let's require explicit arguments for generic functions.
            todo!("Implement type inference and substitution")
        }
        TypeKind::GenericFnType {
            params,
            return_type,
            generic_params,
        } if generic_params.is_empty() => {
            call_result_type = *return_type.clone();

            if params.len() != checked_args.len() {
                errors.push(SemanticError::new(
                    SemanticErrorKind::ArgumentCountMismatch {
                        expected: params.len(),
                        received: checked_args.len(),
                    },
                    expr_span,
                ));
            } else {
                for (param, arg) in params.iter().zip(checked_args.iter()) {
                    if !check_is_assignable(&arg.expr_type, &param.constraint) {
                        errors.push(SemanticError::new(
                            SemanticErrorKind::TypeMismatch {
                                expected: param.constraint.clone(),
                                received: arg.expr_type.clone(),
                            },
                            arg.expr_type.unwrap_expr_span(),
                        ));
                    }
                }
            }
        }
        TypeKind::GenericApply { target, type_args } => {
            if let TypeKind::GenericFnType {
                params,
                return_type,
                generic_params,
            } = &target.kind
            {
                if generic_params.len() != type_args.len() {
                    errors.push(SemanticError::new(
                        SemanticErrorKind::GenericArgumentCountMismatch {
                            expected: generic_params.len(),
                            received: type_args.len(),
                        },
                        checked_left.expr_type.unwrap_expr_span(),
                    ));
                } else {
                    // Build substitution map
                    let substitution: GenericSubstitutionMap = generic_params
                        .iter()
                        .map(|gp| gp.identifier.name.clone())
                        .zip(type_args.iter().cloned())
                        .collect();

                    // Substitute parameter and return types
                    let substituted_params: Vec<Type> = params
                        .iter()
                        .map(|p| substitute_generics(&p.constraint, &substitution, errors))
                        .collect();

                    let substituted_return_type =
                        substitute_generics(return_type, &substitution, errors);

                    call_result_type = substituted_return_type;

                    if substituted_params.len() != checked_args.len() {
                        errors.push(SemanticError::new(
                            SemanticErrorKind::ArgumentCountMismatch {
                                expected: substituted_params.len(),
                                received: checked_args.len(),
                            },
                            expr_span,
                        ));
                    } else {
                        for (expected_type, arg) in
                            substituted_params.iter().zip(checked_args.iter())
                        {
                            if !check_is_assignable(&arg.expr_type, expected_type) {
                                errors.push(SemanticError::new(
                                    SemanticErrorKind::TypeMismatch {
                                        expected: expected_type.clone(),
                                        received: arg.expr_type.clone(),
                                    },
                                    arg.expr_type.unwrap_expr_span(),
                                ));
                            }
                        }
                    }
                }
            } else {
                errors.push(SemanticError::new(
                    SemanticErrorKind::CannotCall(*target.clone()),
                    checked_left.expr_type.unwrap_expr_span(),
                ));
            }
        }
        non_callable_type => {
            errors.push(SemanticError::new(
                SemanticErrorKind::CannotCall(checked_left.expr_type.clone()),
                checked_left.expr_type.unwrap_expr_span(),
            ));
        }
    }

    CheckedExpr {
        expr_type: call_result_type,
        kind: CheckedExprKind::FnCall {
            left: Box::new(checked_left),
            args: checked_args,
        },
    }
}
