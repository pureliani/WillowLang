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
    check::{utils::substitute_generics::GenericSubstitutionMap, SemanticChecker, SemanticError},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_fn_call_expr(&mut self, left: Box<Expr>, args: Vec<Expr>, span: Span) -> CheckedExpr {
        let checked_left = self.check_expr(*left);
        let checked_args: Vec<_> = args.into_iter().map(|arg| self.check_expr(arg)).collect();

        let return_type = match &checked_left.ty.kind {
            CheckedTypeKind::FnType(CheckedFnType {
                params,
                return_type,
                generic_params: _,
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
                    let mut substitutions = GenericSubstitutionMap::new();
                    for (fn_param, call_arg_expr) in params.iter().zip(checked_args.iter()) {
                        self.infer_generics(&fn_param.constraint, &call_arg_expr.ty, &mut substitutions);
                    }

                    let substituted_fn_params: Vec<CheckedParam> = params
                        .iter()
                        .map(|p| CheckedParam {
                            id: p.id,
                            identifier: p.identifier,
                            constraint: self.substitute_generics(&p.constraint, &substitutions),
                        })
                        .collect();

                    let mut substituted_return = self.substitute_generics(&return_type, &substitutions);
                    substituted_return.span = span;

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
