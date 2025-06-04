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
        scope::Scope, utils::substitute_generics::GenericSubstitutionMap, SemanticChecker,
        SemanticError, SemanticErrorKind,
    },
};

impl<'a> SemanticChecker<'a> {
    pub fn check_fn_call_expr(
        &mut self,
        left: Box<Expr>,
        args: Vec<Expr>,
        span: Span,
        scope: Rc<RefCell<Scope>>,
    ) -> CheckedExpr {
        let checked_left = self.check_expr(*left, scope.clone());
        let checked_args: Vec<_> = args
            .into_iter()
            .map(|arg| self.check_expr(arg, scope.clone()))
            .collect();

        let mut call_result_type = CheckedType::Unknown;

        match &checked_left.ty {
            CheckedType::FnType {
                params,
                return_type,
                generic_params: _,
            } => {
                if checked_args.len() != params.len() {
                    self.errors.push(SemanticError {
                        kind: SemanticErrorKind::FnArgumentCountMismatch {
                            expected: params.len(),
                            received: checked_args.len(),
                        },
                        span: span,
                    });
                } else {
                    let mut substitutions: GenericSubstitutionMap = HashMap::new();

                    for (param, arg) in params.iter().zip(checked_args.iter()) {
                        self.infer_generics(&param.constraint, &arg.ty, &mut substitutions);
                    }

                    let substituted_return = self.substitute_generics(&return_type, &substitutions);

                    call_result_type = substituted_return;

                    let substituted_params: Vec<CheckedParam> = params
                        .into_iter()
                        .map(|p| CheckedParam {
                            constraint: self.substitute_generics(&p.constraint, &substitutions),
                            identifier: p.identifier,
                        })
                        .collect();

                    for (param, arg) in substituted_params.into_iter().zip(checked_args.iter()) {
                        if !self.check_is_assignable(&arg.ty, &param.constraint) {
                            self.errors.push(SemanticError {
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
                self.errors.push(SemanticError {
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
}
