use std::collections::HashMap;

use crate::{
    ast::checked::{
        checked_declaration::{CheckedParam, CheckedStructDecl},
        checked_type::{Type, TypeKind},
    },
    check::{SemanticError, SemanticErrorKind},
};

use super::union_of::union_of;

pub type GenericSubstitutionMap = HashMap<String, Type>;

pub fn substitute_generics(
    ty: &Type,
    substitution: &GenericSubstitutionMap,
    errors: &mut Vec<SemanticError>,
) -> Type {
    match &ty.kind {
        TypeKind::GenericParam(gp) => substitution
            .get(&gp.identifier.name)
            .cloned()
            .unwrap_or_else(|| {
                let span = ty.unwrap_annotation_span();

                errors.push(SemanticError::new(
                    SemanticErrorKind::UnresolvedGenericParam(gp.identifier.name.clone()),
                    span,
                ));

                Type {
                    kind: TypeKind::Unknown,
                    span: ty.span,
                }
            }),
        TypeKind::FnType {
            params,
            return_type,
            generic_params,
        } => {
            // IMPORTANT: When substituting within a function type, we generally DON'T
            // substitute its *own* generic parameters. Those are bound locally.
            // We only substitute types that came from an outer scope's substitution.
            let substituted_params = params
                .iter()
                .map(|p| CheckedParam {
                    identifier: p.identifier.clone(),
                    constraint: substitute_generics(&p.constraint, substitution, errors),
                })
                .collect();

            let substituted_return_type = substitute_generics(return_type, substitution, errors);

            Type {
                kind: TypeKind::FnType {
                    params: substituted_params,
                    return_type: Box::new(substituted_return_type),
                    generic_params: generic_params.clone(), // Keep original generic params
                },
                span: ty.span,
            }
        }
        TypeKind::Struct(decl) => {
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

            Type {
                kind: TypeKind::Struct(CheckedStructDecl {
                    properties: substituted_props,
                    ..decl.clone()
                }),
                span: ty.span,
            }
        }
        TypeKind::Array { item_type, size } => Type {
            kind: TypeKind::Array {
                item_type: Box::new(substitute_generics(item_type, substitution, errors)),
                size: *size,
            },
            span: ty.span,
        },
        TypeKind::Union(items) => {
            let substituted_items: Vec<Type> = items
                .iter()
                .map(|t| substitute_generics(t, substitution, errors))
                .collect();
            // Re-apply union_of logic to simplify the result
            union_of(&substituted_items)
        }
        TypeKind::I8
        | TypeKind::I16
        | TypeKind::I32
        | TypeKind::I64
        | TypeKind::ISize
        | TypeKind::U8
        | TypeKind::U16
        | TypeKind::U32
        | TypeKind::U64
        | TypeKind::USize
        | TypeKind::F32
        | TypeKind::F64
        | TypeKind::Bool
        | TypeKind::Char
        | TypeKind::Void
        | TypeKind::Null
        | TypeKind::Unknown
        | TypeKind::TypeAlias(_)
        | TypeKind::Enum(_) => ty.clone(),
        TypeKind::GenericApply { target, type_args } => todo!(),
    }
}
