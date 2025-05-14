use std::collections::HashMap;

use crate::{
    ast::checked::{
        checked_declaration::{CheckedParam, CheckedStructDecl, CheckedTypeAliasDecl},
        checked_type::{CheckedTypeX, CheckedType},
    },
    check::{SemanticError, SemanticErrorKind},
};

use super::union_of::union_of;

pub type GenericSubstitutionMap = HashMap<String, CheckedTypeX>;

pub fn substitute_generics(
    ty: &CheckedTypeX,
    substitution: &GenericSubstitutionMap,
    errors: &mut Vec<SemanticError>,
) -> CheckedTypeX {
    match &ty.kind {
        CheckedType::GenericParam(gp) => substitution
            .get(&gp.identifier.name)
            .cloned()
            .unwrap_or_else(|| {
                let span = ty.unwrap_annotation_span();

                errors.push(SemanticError::new(
                    SemanticErrorKind::UnresolvedGenericParam(gp.identifier.name.clone()),
                    span,
                ));

                CheckedTypeX {
                    kind: CheckedType::Unknown,
                    span: ty.span,
                }
            }),
        CheckedType::GenericFnType {
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

            CheckedTypeX {
                kind: CheckedType::FnType {
                    params: substituted_params,
                    return_type: Box::new(substituted_return_type),
                },
                span: ty.span,
            }
        }
        CheckedType::FnType {
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

            CheckedTypeX {
                kind: CheckedType::FnType {
                    params: substituted_params,
                    return_type: Box::new(substituted_return_type),
                },
                span: ty.span,
            }
        }
        CheckedType::GenericStructDecl(decl) => {
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

            CheckedTypeX {
                kind: CheckedType::StructDecl(CheckedStructDecl {
                    properties: substituted_props,
                    documentation: decl.documentation.clone(),
                    identifier: decl.identifier.clone(), // maybe we should rename this?
                }),
                span: ty.span,
            }
        }
        CheckedType::GenericTypeAliasDecl(decl) => {
            let substituted_value = substitute_generics(&decl.value, substitution, errors);

            CheckedTypeX {
                kind: CheckedType::TypeAliasDecl(CheckedTypeAliasDecl {
                    value: Box::new(substituted_value),
                    documentation: decl.documentation.clone(),
                    identifier: decl.identifier.clone(), // maybe we should rename this?
                }),
                span: ty.span,
            }
        }
        CheckedType::Array { item_type, size } => CheckedTypeX {
            kind: CheckedType::Array {
                item_type: Box::new(substitute_generics(item_type, substitution, errors)),
                size: *size,
            },
            span: ty.span,
        },
        CheckedType::Union(items) => {
            let substituted_items = items
                .iter()
                .map(|t| substitute_generics(t, substitution, errors));

            // Re-apply union_of logic to simplify the result
            union_of(substituted_items)
        }
        CheckedType::I8
        | CheckedType::I16
        | CheckedType::I32
        | CheckedType::I64
        | CheckedType::ISize
        | CheckedType::U8
        | CheckedType::U16
        | CheckedType::U32
        | CheckedType::U64
        | CheckedType::USize
        | CheckedType::F32
        | CheckedType::F64
        | CheckedType::Bool
        | CheckedType::Char
        | CheckedType::Void
        | CheckedType::Null
        | CheckedType::Unknown
        | CheckedType::TypeAliasDecl(_)
        | CheckedType::StructDecl(_)
        | CheckedType::Enum(_) => ty.clone(),
    }
}
