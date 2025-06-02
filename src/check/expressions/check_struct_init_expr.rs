use std::{cell::RefCell, collections::HashSet, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_declaration::{CheckedGenericStructDecl, CheckedStructDecl},
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::CheckedType,
        },
        IdentifierNode, Span,
    },
    check::{
        check_expr::check_expr, scope::Scope, utils::check_is_assignable::check_is_assignable,
        SemanticError, SemanticErrorKind,
    },
};

pub fn check_struct_init_expr(
    left: Box<Expr>,
    fields: Vec<(IdentifierNode, Expr)>,
    span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_args: Vec<(IdentifierNode, CheckedExpr)> = fields
        .into_iter()
        .map(|f| (f.0, check_expr(f.1, errors, scope.clone())))
        .collect();

    // TODO: make sure that checked_left.kind is actually an identifier or genericapply wrapper around an identifier

    match &checked_left.ty {
        CheckedType::GenericStructDecl(CheckedGenericStructDecl {
            identifier,
            properties,
            documentation,
            generic_params,
        }) => {
            todo!()
        }
        CheckedType::StructDecl(CheckedStructDecl { properties, .. }) => {
            let mut uninitialized_props: HashSet<_> =
                properties.iter().map(|p| p.identifier.name).collect();

            checked_args.iter().for_each(|a| {
                let span = a.0.span;
                let property = properties.iter().find(|p| p.identifier.name == a.0.name);

                match property {
                    None => {
                        errors.push(SemanticError {
                            kind: SemanticErrorKind::UnknownStructPropertyInitializer(a.0),
                            span,
                        });
                    }
                    Some(p) => {
                        if !uninitialized_props.remove(&a.0.name) {
                            errors.push(SemanticError {
                                kind: SemanticErrorKind::DuplicateStructPropertyInitializer(a.0),
                                span,
                            })
                        } else {
                            if !check_is_assignable(&a.1.ty, &p.constraint) {
                                errors.push(SemanticError {
                                    kind: SemanticErrorKind::TypeMismatch {
                                        expected: p.constraint.clone(),
                                        received: a.1.ty.clone(),
                                    },
                                    span: a.1.span,
                                });
                            }
                        }
                    }
                }
            });

            if uninitialized_props.len() > 0 {
                errors.push(SemanticError {
                    kind: SemanticErrorKind::MissingStructPropertyInitializer(uninitialized_props),
                    span,
                });
            }

            CheckedExpr {
                ty: checked_left.ty.clone(),
                kind: CheckedExprKind::StructInit {
                    left: Box::new(checked_left),
                    fields: checked_args,
                },
                span,
            }
        }
        _ => {
            errors.push(SemanticError {
                kind: SemanticErrorKind::CannotApplyStructInitializer,
                span: checked_left.span,
            });

            CheckedExpr {
                ty: checked_left.ty.clone(),
                kind: CheckedExprKind::StructInit {
                    left: Box::new(checked_left),
                    fields: checked_args,
                },
                span,
            }
        }
    }
}
