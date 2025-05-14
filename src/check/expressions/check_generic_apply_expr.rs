use std::{cell::RefCell, iter, rc::Rc};

use crate::{
    ast::{
        base::{base_expression::Expr, base_type::TypeAnnotation},
        checked::{
            checked_declaration::{
                CheckedGenericParam, CheckedParam, CheckedGenericStructDecl, CheckedStructDecl,
            },
            checked_expression::{CheckedExpr, CheckedExprKind},
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

    let mut get_substitutions = |generic_params: Vec<CheckedGenericParam>,
                                 type_args: Vec<CheckedType>|
     -> GenericSubstitutionMap {
        if generic_params.len() != type_args.len() {
            errors.push(SemanticError::new(
                SemanticErrorKind::GenericArgumentCountMismatch {
                    expected: generic_params.len(),
                    received: type_args.len(),
                },
                expr_span,
            ));
        } else {
            generic_params
                .iter()
                .zip(type_args.iter())
                .for_each(|(gp, ta)| {
                    if let Some(constraint) = &gp.constraint {
                        if !check_is_assignable(ta, constraint) {
                            errors.push(SemanticError::new(
                                SemanticErrorKind::TypeMismatch {
                                    expected: *constraint.clone(),
                                    received: ta.clone(),
                                },
                                ta.unwrap_annotation_span(),
                            ));
                        }
                    }
                });
        };

        let substitutions: GenericSubstitutionMap = generic_params
            .into_iter()
            .map(|gp| gp.identifier.name.clone())
            .zip(type_args.into_iter().chain(iter::repeat(CheckedType {
                kind: CheckedTypeKind::Unknown,
                span: TypeSpan::None,
            })))
            .collect();

        substitutions
    };

    let (type_kind, substitutions) = match checked_left.expr_type.kind.clone() {
        CheckedTypeKind::GenericFnType {
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
                CheckedTypeKind::FnType {
                    params: substituted_params,
                    return_type: Box::new(substituted_return_type),
                },
                substitutions,
            )
        }
        CheckedTypeKind::GenericStructDecl(CheckedGenericStructDecl {
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
                CheckedTypeKind::StructDecl(CheckedStructDecl {
                    documentation: documentation.clone(),
                    identifier: identifier.clone(),
                    properties: substituted_properties,
                }),
                substitutions,
            )
        }
        _ => {
            errors.push(SemanticError::new(
                SemanticErrorKind::CannotApplyTypeArguments {
                    to: checked_left.expr_type.clone(),
                },
                expr_span,
            ));

            (CheckedTypeKind::Unknown, GenericSubstitutionMap::new())
        }
    };

    CheckedExpr {
        expr_type: CheckedType {
            kind: type_kind,
            span: TypeSpan::Expr(expr_span),
        },
        kind: CheckedExprKind::GenericSpecialization {
            target: Box::new(checked_left),
            substitutions,
        },
    }
}
