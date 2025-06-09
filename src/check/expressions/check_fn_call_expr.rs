use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_declaration::{CheckedFnType, CheckedParam},
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
        Span,
    },
    check::{
        expressions::check_generic_apply_expr::GenericArgumentSource, scope::Scope,
        utils::substitute_generics::GenericSubstitutionMap, SemanticChecker, SemanticError,
    },
};

impl<'a> SemanticChecker<'a> {
    pub fn check_fn_call_expr(&mut self, left: Box<Expr>, args: Vec<Expr>, span: Span, scope: Rc<RefCell<Scope>>) -> CheckedExpr {
        let checked_left = self.check_expr(*left, scope.clone());
        let checked_args: Vec<_> = args.into_iter().map(|arg| self.check_expr(arg, scope.clone())).collect();

        let return_type = match &checked_left.ty.kind {
            CheckedTypeKind::FnType(CheckedFnType {
                params,
                return_type,
                generic_params,
                ..
            }) => {
                if checked_args.len() != params.len() {
                    self.errors.push(SemanticError::FnArgumentCountMismatch {
                        expected: params.len(),
                        received: checked_args.len(),
                        span: checked_left.ty.span,
                    });

                    CheckedType {
                        kind: CheckedTypeKind::Unknown,
                        span,
                    }
                } else {
                    let mut inferred_substitutions = GenericSubstitutionMap::new();
                    if !generic_params.is_empty() {
                        for (fn_param, call_arg_expr) in params.iter().zip(checked_args.iter()) {
                            self.infer_generics(&fn_param.constraint, &call_arg_expr.ty, &mut inferred_substitutions);
                        }
                    }

                    let substitutions_opt = self.build_substitution_map(
                        generic_params,
                        GenericArgumentSource::Inferred {
                            substitutions: &inferred_substitutions,
                        },
                        span,
                    );

                    if let Some(substitutions) = substitutions_opt {
                        let mut substituted_return = self.substitute_generics(&return_type, &substitutions);

                        let substituted_fn_params: Vec<CheckedParam> = params
                            .iter()
                            .map(|p| CheckedParam {
                                identifier: p.identifier,
                                constraint: self.substitute_generics(&p.constraint, &substitutions),
                            })
                            .collect();

                        for (call_arg_expr, substituted_param) in checked_args.iter().zip(substituted_fn_params.iter()) {
                            if !self.check_is_assignable(&call_arg_expr.ty, &substituted_param.constraint) {
                                self.errors.push(SemanticError::TypeMismatch {
                                    expected: substituted_param.constraint.clone(),
                                    received: call_arg_expr.ty.clone(),
                                });

                                substituted_return.kind = CheckedTypeKind::Unknown;
                            }
                        }

                        substituted_return
                    } else {
                        CheckedType {
                            kind: CheckedTypeKind::Unknown,
                            span,
                        }
                    }
                }
            }
            _ => {
                self.errors.push(SemanticError::CannotCall {
                    target: checked_left.ty.clone(),
                });

                CheckedType {
                    kind: CheckedTypeKind::Unknown,
                    span,
                }
            }
        };

        CheckedExpr {
            ty: return_type,
            kind: CheckedExprKind::FnCall {
                left: Box::new(checked_left),
                args: checked_args,
            },
        }
    }
}
