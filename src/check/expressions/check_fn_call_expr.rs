use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_declaration::CheckedParam,
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::CheckedType,
        },
        Span,
    },
    check::{
        check_expr::check_expr,
        scope::Scope,
        utils::{
            check_is_assignable::check_is_assignable,
            infer_generics::infer_generics,
            substitute_generics::{substitute_generics, GenericSubstitutionMap},
        },
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
                errors.push(SemanticError {
                    kind: SemanticErrorKind::FnArgumentCountMismatch {
                        expected: params.len(),
                        received: checked_args.len(),
                    },
                    span: span,
                });
            } else {
                for (param, arg) in params.iter().zip(checked_args.iter()) {
                    if !check_is_assignable(&arg.ty, &param.constraint) {
                        errors.push(SemanticError {
                            kind: SemanticErrorKind::TypeMismatch {
                                expected: param.constraint.clone(),
                                received: arg.ty.clone(),
                            },
                            span: arg.span,
                        });
                    }
                }
            }
        }
        CheckedType::GenericFnType {
            params,
            return_type,
            generic_params: _,
        } => {
            if checked_args.len() != params.len() {
                errors.push(SemanticError {
                    kind: SemanticErrorKind::FnArgumentCountMismatch {
                        expected: params.len(),
                        received: checked_args.len(),
                    },
                    span: span,
                });
            } else {
                let mut substitution: GenericSubstitutionMap = HashMap::new();

                for (param, arg) in params.iter().zip(checked_args.iter()) {
                    infer_generics(&param.constraint, &arg.ty, &mut substitution, errors);
                }

                let substituted_return = substitute_generics(&return_type, &substitution, errors);

                call_result_type = substituted_return;

                let substituted_params: Vec<CheckedParam> = params
                    .into_iter()
                    .map(|p| CheckedParam {
                        constraint: substitute_generics(&p.constraint, &substitution, errors),
                        identifier: p.identifier,
                    })
                    .collect();

                for (param, arg) in substituted_params.into_iter().zip(checked_args.iter()) {
                    if !check_is_assignable(&arg.ty, &param.constraint) {
                        errors.push(SemanticError {
                            kind: SemanticErrorKind::TypeMismatch {
                                expected: param.constraint,
                                received: arg.ty.clone(),
                            },
                            span: arg.span,
                        });
                    }
                }
            }
        }
        non_callable_type => {
            errors.push(SemanticError {
                kind: SemanticErrorKind::CannotCall(non_callable_type.clone()),
                span: checked_left.span,
            });
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
