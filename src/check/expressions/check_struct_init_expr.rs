use std::{cell::RefCell, collections::HashSet, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_declaration::{CheckedParam, CheckedStructDecl},
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::CheckedType,
        },
        IdentifierNode, Span,
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
    compile::string_interner::InternerId,
};

pub fn check_struct_init_expr(
    left_expr: Box<Expr>,
    fields: Vec<(IdentifierNode, Expr)>,
    span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_left = check_expr(*left_expr, errors, scope.clone());

    let checked_args: Vec<(IdentifierNode, CheckedExpr)> = fields
        .into_iter()
        .map(|(ident, expr)| (ident, check_expr(expr, errors, scope.clone())))
        .collect();

    // TODO: make sure that checked_left.kind is actually an identifier or genericapply wrapper around an identifier

    match &checked_left.ty {
        CheckedType::StructDecl(decl) => {
            let mut substitutions = GenericSubstitutionMap::new();
            let mut uninitialized_props: HashSet<InternerId> =
                decl.properties.iter().map(|p| p.identifier.name).collect();

            for (arg_ident, arg_expr) in checked_args.iter() {
                let prop = decl
                    .properties
                    .iter()
                    .find(|p| p.identifier.name == arg_ident.name);

                match prop {
                    None => {
                        errors.push(SemanticError {
                            kind: SemanticErrorKind::UnknownStructPropertyInitializer(
                                arg_ident.clone(),
                            ),
                            span: arg_ident.span,
                        });
                    }
                    Some(prop) => {
                        let already_initialized = !uninitialized_props.remove(&arg_ident.name);
                        if already_initialized {
                            errors.push(SemanticError {
                                kind: SemanticErrorKind::DuplicateStructPropertyInitializer(
                                    arg_ident.clone(),
                                ),
                                span: arg_ident.span,
                            });
                        } else {
                            infer_generics(
                                &prop.constraint,
                                &arg_expr.ty,
                                &mut substitutions,
                                errors,
                            );
                        }
                    }
                }
            }

            let final_properties: Vec<CheckedParam> = decl
                .properties
                .iter()
                .map(|p| CheckedParam {
                    identifier: p.identifier, // Keep original identifier node from declaration
                    constraint: substitute_generics(&p.constraint, &substitutions, errors),
                })
                .collect();

            for (arg_ident, arg_expr) in &checked_args {
                if let Some(final_prop) = final_properties
                    .iter()
                    .find(|p| p.identifier.name == arg_ident.name)
                {
                    if !check_is_assignable(&arg_expr.ty, &final_prop.constraint) {
                        errors.push(SemanticError {
                            kind: SemanticErrorKind::TypeMismatch {
                                expected: final_prop.constraint.clone(),
                                received: arg_expr.ty.clone(),
                            },
                            span: arg_expr.span,
                        });
                    }
                }
            }

            if !uninitialized_props.is_empty() {
                errors.push(SemanticError {
                    kind: SemanticErrorKind::MissingStructPropertyInitializer(uninitialized_props),
                    span,
                });
            }

            let final_struct_type = CheckedType::StructDecl(CheckedStructDecl {
                identifier: decl.identifier,
                documentation: decl.documentation.clone(),
                properties: final_properties,
                generic_params: vec![],
            });

            CheckedExpr {
                ty: final_struct_type,
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
                ty: CheckedType::Unknown,
                kind: CheckedExprKind::StructInit {
                    left: Box::new(checked_left),
                    fields: checked_args,
                },
                span,
            }
        }
    }
}
