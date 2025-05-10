use std::collections::HashMap;

use crate::{
    ast::checked::{
        checked_declaration::{CheckedParam, StructDecl, TypeAliasDecl},
        checked_type::{CheckedType, CheckedTypeKind},
    },
    check::{SemanticError, SemanticErrorKind},
};

use super::union_of::union_of;

pub type GenericSubstitutionMap = HashMap<String, CheckedType>;

pub fn substitute_generics(
    ty: &CheckedType,
    substitution: &GenericSubstitutionMap,
    errors: &mut Vec<SemanticError>,
) -> CheckedType {
    match &ty.kind {
        CheckedTypeKind::GenericParam(gp) => substitution
            .get(&gp.identifier.name)
            .cloned()
            .unwrap_or_else(|| {
                let span = ty.unwrap_annotation_span();

                errors.push(SemanticError::new(
                    SemanticErrorKind::UnresolvedGenericParam(gp.identifier.name.clone()),
                    span,
                ));

                CheckedType {
                    kind: CheckedTypeKind::Unknown,
                    span: ty.span,
                }
            }),
        CheckedTypeKind::GenericFnType {
            params,
            return_type,
            generic_params: _, // not needed
        } => {
            // IMPORTANT: When substituting within a function type, we DON'T
            // substitute its *own* generic parameters.
            // We only substitute types that came from an outer scope's substitution.
            let substituted_params = params
                .iter()
                .map(|p| CheckedParam {
                    identifier: p.identifier.clone(),
                    constraint: substitute_generics(&p.constraint, substitution, errors),
                })
                .collect();

            let substituted_return_type = substitute_generics(return_type, substitution, errors);

            CheckedType {
                kind: CheckedTypeKind::FnType {
                    params: substituted_params,
                    return_type: Box::new(substituted_return_type),
                },
                span: ty.span,
            }
        }
        CheckedTypeKind::FnType {
            params,
            return_type,
        } => {
            // This case could be needed when a closure uses generic parameter which was defined by parent

            let substituted_params = params
                .iter()
                .map(|p| CheckedParam {
                    identifier: p.identifier.clone(),
                    constraint: substitute_generics(&p.constraint, substitution, errors),
                })
                .collect();

            let substituted_return_type = substitute_generics(return_type, substitution, errors);

            CheckedType {
                kind: CheckedTypeKind::FnType {
                    params: substituted_params,
                    return_type: Box::new(substituted_return_type),
                },
                span: ty.span,
            }
        }
        CheckedTypeKind::GenericStructDecl(decl) => {
            // Similar to FnType, a struct definition's generic params are local.
            // We substitute types *within* its properties if those types refer
            // to generics from the *outer* substitution context.
            let substituted_props = decl
                .properties
                .iter()
                .map(|p| CheckedParam {
                    identifier: p.identifier.clone(),
                    constraint: substitute_generics(&p.constraint, substitution, errors),
                })
                .collect();

            CheckedType {
                kind: CheckedTypeKind::StructDecl(StructDecl {
                    properties: substituted_props,
                    documentation: decl.documentation.clone(),
                    identifier: decl.identifier.clone(), // maybe we should rename this?
                }),
                span: ty.span,
            }
        }
        CheckedTypeKind::GenericTypeAliasDecl(decl) => {
            let substituted_value = substitute_generics(&decl.value, substitution, errors);

            CheckedType {
                kind: CheckedTypeKind::TypeAliasDecl(TypeAliasDecl {
                    value: Box::new(substituted_value),
                    documentation: decl.documentation.clone(),
                    identifier: decl.identifier.clone(), // maybe we should rename this?
                }),
                span: ty.span,
            }
        }
        CheckedTypeKind::Array { item_type, size } => CheckedType {
            kind: CheckedTypeKind::Array {
                item_type: Box::new(substitute_generics(item_type, substitution, errors)),
                size: *size,
            },
            span: ty.span,
        },
        CheckedTypeKind::Union(items) => {
            let substituted_items: Vec<CheckedType> = items
                .iter()
                .map(|t| substitute_generics(t, substitution, errors))
                .collect();
            // Re-apply union_of logic to simplify the result
            union_of(&substituted_items)
        }
        CheckedTypeKind::I8
        | CheckedTypeKind::I16
        | CheckedTypeKind::I32
        | CheckedTypeKind::I64
        | CheckedTypeKind::ISize
        | CheckedTypeKind::U8
        | CheckedTypeKind::U16
        | CheckedTypeKind::U32
        | CheckedTypeKind::U64
        | CheckedTypeKind::USize
        | CheckedTypeKind::F32
        | CheckedTypeKind::F64
        | CheckedTypeKind::Bool
        | CheckedTypeKind::Char
        | CheckedTypeKind::Void
        | CheckedTypeKind::Null
        | CheckedTypeKind::Unknown
        | CheckedTypeKind::TypeAliasDecl(_)
        | CheckedTypeKind::StructDecl(_)
        | CheckedTypeKind::Enum(_) => ty.clone(),
    }
}
