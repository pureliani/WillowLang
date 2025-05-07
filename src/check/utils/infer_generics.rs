use crate::{
    ast::checked::checked_type::{Type, TypeKind},
    check::{SemanticError, SemanticErrorKind},
};

use super::substitute_generics::GenericSubstitutionMap;

pub fn infer_generics(
    expected: &Type,
    received: &Type,
    substitution: &mut GenericSubstitutionMap,
    errors: &mut Vec<SemanticError>,
) {
    match (&expected.kind, &received.kind) {
        // Handle generics
        (TypeKind::GenericParam(gp), received_kind) => {
            let name = &gp.identifier.name;
            if let Some(existing) = substitution.get(name) {
                if &existing.kind != received_kind {
                    errors.push(SemanticError::new(
                        SemanticErrorKind::ConflictingGenericBinding {
                            existing: existing.clone(),
                            new: received.clone(),
                        },
                        received.unwrap_annotation_span(),
                    ));
                }
            } else {
                substitution.insert(name.clone(), received.clone());
            }
        }
        // Recursively check components (arrays, structs, etc.)
        (
            TypeKind::Array {
                item_type: maybe_generic,
                ..
            },
            TypeKind::Array {
                item_type: concrete,
                ..
            },
        ) => {
            infer_generics(maybe_generic, concrete, substitution, errors);
        }
        (TypeKind::GenericStructDecl(maybe_generic), TypeKind::GenericStructDecl(concrete)) => {
            for (maybe_generic_prop, concrete_prop) in maybe_generic
                .properties
                .iter()
                .zip(concrete.properties.iter())
            {
                infer_generics(
                    &maybe_generic_prop.constraint,
                    &concrete_prop.constraint,
                    substitution,
                    errors,
                );
            }
        }
        (
            TypeKind::GenericFnType {
                params: maybe_generic_params,
                return_type: maybe_generic_return_type,
                generic_params: _,
            },
            TypeKind::GenericFnType {
                params: concrete_params,
                return_type: concrete_return_type,
                generic_params: _,
            },
        ) => {
            todo!("Implement inferring types for functions")
        }
        (TypeKind::Union(maybe_generic), TypeKind::Union(concrete)) => {
            todo!("Implement inferring types for unions")
        }
        _ => {}
    }
}
