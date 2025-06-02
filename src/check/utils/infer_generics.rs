use crate::{
    ast::checked::checked_type::CheckedType,
    check::{SemanticError, SemanticErrorKind},
};

use super::substitute_generics::GenericSubstitutionMap;

pub fn infer_generics(
    expected: &CheckedType,
    received: &CheckedType,
    substitution: &mut GenericSubstitutionMap,
    errors: &mut Vec<SemanticError>,
) {
    match (expected, received) {
        // Handle generics
        (CheckedType::GenericParam(expected_generic_param), received_kind) => {
            let name = &expected_generic_param.identifier.name;
            if let Some(existing) = substitution.get(name) {
                if existing != received_kind {
                    errors.push(SemanticError {
                        kind: SemanticErrorKind::ConflictingGenericBinding {
                            identifier: expected_generic_param.identifier,
                            existing: existing.clone(),
                            new: received.clone(),
                        },
                        span: expected_generic_param.identifier.span,
                    });
                }
            } else {
                substitution.insert(name.clone(), received.clone());
            }
        }
        // Recursively check components (arrays, structs, etc.)
        (
            CheckedType::Array {
                item_type: expected_item_type,
                ..
            },
            CheckedType::Array {
                item_type: received_item_type,
                ..
            },
        ) => {
            infer_generics(expected_item_type, received_item_type, substitution, errors);
        }
        (CheckedType::StructDecl(expected), CheckedType::StructDecl(received)) => {
            for (generic_prop, concrete_prop) in
                expected.properties.iter().zip(received.properties.iter())
            {
                infer_generics(
                    &generic_prop.constraint,
                    &concrete_prop.constraint,
                    substitution,
                    errors,
                );
            }
        }
        (
            CheckedType::FnType {
                params: generic_params,
                return_type: generic_return_type,
                generic_params: _,
            },
            CheckedType::FnType {
                params: concrete_params,
                return_type: concrete_return_type,
                generic_params: _,
            },
        ) => {
            for (generic_param, concrete_param) in generic_params.iter().zip(concrete_params.iter())
            {
                infer_generics(
                    &generic_param.constraint,
                    &concrete_param.constraint,
                    substitution,
                    errors,
                );
            }

            infer_generics(
                &generic_return_type,
                &concrete_return_type,
                substitution,
                errors,
            );
        }
        (CheckedType::Union(expected_union), CheckedType::Union(received_union)) => {}
        (CheckedType::Union(expected_union), non_union) => {}
        _ => {}
    }
}
