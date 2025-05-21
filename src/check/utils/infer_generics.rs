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
    match (&expected, &received) {
        // Handle generics
        (CheckedType::GenericParam(gp), received_kind) => {
            let name = &gp.identifier.name;
            if let Some(existing) = substitution.get(name) {
                if &existing != received_kind {
                    errors.push(SemanticError::new(
                        SemanticErrorKind::ConflictingGenericBinding {
                            existing: existing.clone(),
                            new: received.clone(),
                        },
                        gp.identifier.span,
                    ));
                }
            } else {
                substitution.insert(name.clone(), received.clone());
            }
        }
        // Recursively check components (arrays, structs, etc.)
        (
            CheckedType::Array {
                item_type: maybe_generic,
                ..
            },
            CheckedType::Array {
                item_type: concrete,
                ..
            },
        ) => {
            infer_generics(maybe_generic, concrete, substitution, errors);
        }
        (CheckedType::GenericStructDecl(generic), CheckedType::StructDecl(concrete)) => {
            for (generic_prop, concrete_prop) in
                generic.properties.iter().zip(concrete.properties.iter())
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
            CheckedType::GenericFnType {
                params: generic_params,
                return_type: generic_return_type,
                generic_params: _,
            },
            CheckedType::FnType {
                params: concrete_params,
                return_type: concrete_return_type,
            },
        )
        | (
            CheckedType::FnType {
                params: generic_params,
                return_type: generic_return_type,
            },
            CheckedType::FnType {
                params: concrete_params,
                return_type: concrete_return_type,
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
