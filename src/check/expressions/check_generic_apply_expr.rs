use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::{base_expression::Expr, base_type::TypeAnnotation},
        checked::{
            checked_declaration::{
                CheckedGenericParam, CheckedParam, GenericStructDecl, GenericTypeAliasDecl,
                StructDecl,
            },
            checked_expression::{CheckedBlockContents, CheckedExpr, CheckedExprKind, GenericFn},
            checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
        },
        Span,
    },
    check::{
        check_expr::check_expr,
        scope::Scope,
        utils::{
            check_is_assignable::check_is_assignable,
            substitute_generics::{substitute_generics, GenericSubstitutionMap},
            type_annotation_to_semantic::check_type,
        },
        SemanticError, SemanticErrorKind,
    },
};

pub fn check_generic_apply_expr(
    left: Box<Expr>,
    args: Vec<TypeAnnotation>,
    expr_span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_left = check_expr(*left, errors, scope.clone());
    let type_args: Vec<_> = args
        .into_iter()
        .map(|type_arg| check_type(&type_arg, errors, scope.clone()))
        .collect();

    let mut check_type_args = |generic_params: Vec<CheckedGenericParam>,
                               type_args: Vec<CheckedType>|
     -> Option<GenericSubstitutionMap> {
        let is_valid_usage = if generic_params.len() != type_args.len() {
            errors.push(SemanticError::new(
                SemanticErrorKind::GenericArgumentCountMismatch {
                    expected: generic_params.len(),
                    received: type_args.len(),
                },
                expr_span,
            ));

            false
        } else {
            let are_arguments_assignable =
                generic_params
                    .iter()
                    .zip(type_args.iter())
                    .all(|(gp, ta)| match &gp.constraint {
                        Some(constraint) => {
                            let is_assignable = check_is_assignable(ta, constraint);

                            if !is_assignable {
                                errors.push(SemanticError::new(
                                    SemanticErrorKind::TypeMismatch {
                                        expected: *constraint.clone(),
                                        received: ta.clone(),
                                    },
                                    ta.unwrap_annotation_span(),
                                ));
                            }

                            is_assignable
                        }
                        None => true,
                    });

            are_arguments_assignable
        };

        if !is_valid_usage {
            None
        } else {
            let substitution: GenericSubstitutionMap = generic_params
                .into_iter()
                .map(|gp| gp.identifier.name.clone())
                .zip(type_args.into_iter())
                .collect();

            Some(substitution)
        }
    };

    match checked_left.expr_type.kind {
        CheckedTypeKind::GenericFnType {
            params,
            return_type,
            generic_params,
        } => {
            if let Some(substitution) = check_type_args(generic_params, type_args) {
                let substituted_params: Vec<_> = params
                    .into_iter()
                    .map(|p| CheckedParam {
                        constraint: substitute_generics(&p.constraint, &substitution, errors),
                        identifier: p.identifier,
                    })
                    .collect();

                let substituted_return_type =
                    substitute_generics(&return_type, &substitution, errors);

                // Expressions that can potentially result in function type
                let substituted_body = match checked_left.kind {
                    CheckedExprKind::Identifier(id) => {
                        todo!()
                    }
                    CheckedExprKind::If {
                        condition,
                        then_branch,
                        else_if_branches,
                        else_branch,
                    } => {
                        todo!()
                    }
                    CheckedExprKind::TypeCast { left, target } => {
                        todo!()
                    }
                    CheckedExprKind::Block(CheckedBlockContents { final_expr, .. }) => {
                        todo!()
                    }
                    CheckedExprKind::GenericFn(GenericFn {
                        params,
                        body,
                        return_type,
                        generic_params,
                    }) => {
                        todo!()
                    }
                    CheckedExprKind::FnCall { left, args } => {
                        todo!()
                    }
                    CheckedExprKind::Access { left, field } => {
                        todo!()
                    }
                    _ => {
                        unreachable!()
                    }
                };

                // CheckedExpr {
                //     kind: CheckedExprKind::Fn {
                //         params: substituted_params,
                //         body: (),
                //         return_type: substituted_return_type,
                //     },
                // }
            } else {
                todo!("return default CheckedExpr")
            }
        }
        CheckedTypeKind::GenericStructDecl(GenericStructDecl {
            identifier,
            documentation,
            generic_params,
            properties,
        }) => {
            if let Some(substitution) = check_type_args(generic_params, type_args) {
                let substituted_properties: Vec<_> = properties
                    .into_iter()
                    .map(|p| CheckedParam {
                        constraint: substitute_generics(&p.constraint, &substitution, errors),
                        identifier: p.identifier,
                    })
                    .collect();

                let new_id: String = "placeholder".to_string();

                CheckedExpr {
                    expr_type: CheckedType {
                        kind: CheckedTypeKind::StructDecl(StructDecl {
                            documentation,
                            identifier: new_id,
                            properties: substituted_properties,
                        }),
                        span: TypeSpan::Expr(expr_span),
                    },
                }
            } else {
                todo!("return default CheckedExpr")
            }
        }
        CheckedTypeKind::GenericTypeAliasDecl(GenericTypeAliasDecl {
            identifier,
            documentation,
            generic_params,
            value,
        }) => {
            if let Some(substitution) = check_type_args(generic_params, type_args) {
                let substituted_value = substitute_generics(&value, &substitution, errors);

                let new_id: String = "placeholder".to_string();

                CheckedExpr {
                    expr_type: CheckedType {
                        kind: CheckedTypeKind::StructDecl(StructDecl {
                            documentation,
                            identifier: new_id,
                            properties: substituted_properties,
                        }),
                        span: TypeSpan::Expr(expr_span),
                    },
                }
            } else {
                todo!("return default CheckedExpr")
            }
        }
        _ => {
            errors.push(SemanticError::new(
                SemanticErrorKind::CannotApplyTypeArguments {
                    to: checked_left.expr_type,
                },
                expr_span,
            ));
        }
    }

    todo!()
}
