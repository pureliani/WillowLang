use std::{cell::RefCell, iter, rc::Rc};

use crate::{
    ast::{
        base::{base_expression::Expr, base_type::TypeAnnotation},
        checked::{
            checked_declaration::{
                CheckedGenericParam, CheckedGenericStructDecl, CheckedParam, CheckedStructDecl,
            },
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
            substitute_generics::{substitute_generics, GenericSubstitutionMap},
            type_annotation_to_semantic::check_type,
        },
        SemanticError, SemanticErrorKind,
    },
};

pub fn check_generic_apply_expr(
    left: Box<Expr>,
    args: Vec<TypeAnnotation>,
    span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_left = check_expr(*left, errors, scope.clone());
    let type_args: Vec<_> = args
        .into_iter()
        .map(|type_arg| (type_arg.span, check_type(&type_arg, errors, scope.clone())))
        .collect();

    let mut get_substitutions = |generic_params: Vec<CheckedGenericParam>,
                                 type_args: Vec<(Span, CheckedType)>|
     -> GenericSubstitutionMap {
        if generic_params.len() != type_args.len() {
            errors.push(SemanticError {
                kind: SemanticErrorKind::GenericArgumentCountMismatch {
                    expected: generic_params.len(),
                    received: type_args.len(),
                },
                span,
            });
        } else {
            generic_params
                .iter()
                .zip(type_args.iter())
                .for_each(|(gp, ta)| {
                    if let Some(constraint) = &gp.constraint {
                        if !check_is_assignable(&ta.1, constraint) {
                            errors.push(SemanticError {
                                kind: SemanticErrorKind::TypeMismatch {
                                    expected: *constraint.clone(),
                                    received: ta.1.clone(),
                                },
                                span: ta.0,
                            });
                        }
                    }
                });
        };

        let substitutions: GenericSubstitutionMap = generic_params
            .into_iter()
            .map(|gp| gp.identifier.name.clone())
            .zip(
                type_args
                    .into_iter()
                    .map(|ta| ta.1)
                    .chain(iter::repeat(CheckedType::Unknown)),
            )
            .collect();

        substitutions
    };

    let (type_kind, substitutions) = match checked_left.ty.clone() {
        CheckedType::GenericFnType {
            params,
            return_type,
            generic_params,
        } => {
            let substitutions = get_substitutions(generic_params, type_args);

            let substituted_params: Vec<_> = params
                .into_iter()
                .map(|p| CheckedParam {
                    constraint: substitute_generics(&p.constraint, &substitutions, errors),
                    identifier: p.identifier.clone(),
                })
                .collect();

            let substituted_return_type = substitute_generics(&return_type, &substitutions, errors);

            (
                CheckedType::FnType {
                    params: substituted_params,
                    return_type: Box::new(substituted_return_type),
                },
                substitutions,
            )
        }
        CheckedType::GenericStructDecl(CheckedGenericStructDecl {
            identifier,
            documentation,
            generic_params,
            properties,
        }) => {
            let substitutions = get_substitutions(generic_params, type_args);

            let substituted_properties: Vec<_> = properties
                .into_iter()
                .map(|p| CheckedParam {
                    constraint: substitute_generics(&p.constraint, &substitutions, errors),
                    identifier: p.identifier.clone(),
                })
                .collect();

            (
                CheckedType::StructDecl(CheckedStructDecl {
                    documentation: documentation.clone(),
                    identifier: identifier.clone(),
                    properties: substituted_properties,
                }),
                substitutions,
            )
        }
        _ => {
            errors.push(SemanticError {
                kind: SemanticErrorKind::CannotApplyTypeArguments {
                    to: checked_left.ty.clone(),
                },
                span,
            });

            (CheckedType::Unknown, GenericSubstitutionMap::new())
        }
    };

    CheckedExpr {
        span,
        ty: type_kind,
        kind: CheckedExprKind::GenericSpecialization {
            target: Box::new(checked_left),
            substitutions,
        },
    }
}
