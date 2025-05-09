use crate::{
    ast::checked::checked_type::{CheckedType, CheckedTypeKind},
    check::{SemanticError, SemanticErrorKind},
};

use super::substitute_generics::GenericSubstitutionMap;

pub fn infer_generics(
    expected: &CheckedType,
    received: &CheckedType,
    substitution: &mut GenericSubstitutionMap,
    errors: &mut Vec<SemanticError>,
) {
    match (&expected.kind, &received.kind) {
        // Handle generics
        (CheckedTypeKind::GenericParam(gp), received_kind) => {
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
            CheckedTypeKind::Array {
                item_type: maybe_generic,
                ..
            },
            CheckedTypeKind::Array {
                item_type: concrete,
                ..
            },
        ) => {
            infer_generics(maybe_generic, concrete, substitution, errors);
        }
        (
            CheckedTypeKind::GenericStructDecl(maybe_generic),
            CheckedTypeKind::GenericStructDecl(concrete),
        ) => {
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
            CheckedTypeKind::GenericFnType {
                params: maybe_generic_params,
                return_type: maybe_generic_return_type,
                generic_params: _,
            },
            CheckedTypeKind::GenericFnType {
                params: concrete_params,
                return_type: concrete_return_type,
                generic_params: _,
            },
        ) => {
            todo!("Implement inferring types for functions")
        }
        (CheckedTypeKind::Union(maybe_generic), CheckedTypeKind::Union(concrete)) => {
            todo!("Implement inferring types for unions")
        }
        _ => {}
    }
}
