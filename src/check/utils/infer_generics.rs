use crate::{
    ast::{checked::checked_type::CheckedType, Position, Span},
    check::{SemanticError, SemanticErrorKind},
};

use super::substitute_generics::GenericSubstitutionMap;

pub fn infer_generics(
    expected: &CheckedType,
    received: &CheckedType,
    substitution: &mut GenericSubstitutionMap,
) {
    match (expected, received) {
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
                        // TODO: somehow use the span of the received type
                        span: Span {
                            start: Position {
                                line: 0,
                                col: 0,
                                byte_offset: 0,
                            },
                            end: Position {
                                line: 0,
                                col: 0,
                                byte_offset: 0,
                            },
                        },
                    });
                }
            } else {
                substitution.insert(name.clone(), received.clone());
            }
        }
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
        (CheckedType::Union(expected_union), received) => {
            let mut successful_substitutions: Vec<GenericSubstitutionMap> = Vec::new();
            let original_substitution_state = substitution.clone();

            for expected_item in expected_union.iter() {
                let mut current_branch_substitution = original_substitution_state.clone();
                let mut current_branch_errors: Vec<SemanticError> = Vec::new();

                infer_generics(
                    expected_item,
                    received,
                    &mut current_branch_substitution,
                    &mut current_branch_errors,
                );

                if current_branch_errors.is_empty() {
                    successful_substitutions.push(current_branch_substitution);
                }
            }

            if successful_substitutions.is_empty() {
                errors.push(SemanticError {
                    kind: SemanticErrorKind::FailedToInferGenericsInUnion {
                        expected_union: expected_union.clone(),
                        received: received.clone(),
                    },
                    // TODO: fix this once we have the checked type's span
                    span: Span {
                        start: Position {
                            line: 0,
                            col: 0,
                            byte_offset: 0,
                        },
                        end: Position {
                            line: 0,
                            col: 0,
                            byte_offset: 0,
                        },
                    },
                });
            } else if successful_substitutions.len() == 1 {
                *substitution = successful_substitutions.pop().unwrap();
            } else {
                let first_sub = &successful_substitutions[0];
                let all_equivalent = successful_substitutions
                    .iter()
                    .skip(1)
                    .all(|s| s == first_sub);

                if all_equivalent {
                    *substitution = successful_substitutions.pop().unwrap();
                } else {
                    // Example: expected = T | U, received = i32.
                    // Path 1: T=i32 (U unbound by this path) -> sub1
                    // Path 2: U=i32 (T unbound by this path) -> sub2
                    // sub1 != sub2. This is an ambiguity.
                    errors.push(SemanticError {
                        kind: SemanticErrorKind::AmbiguousGenericInferenceForUnion {
                            expected: expected_union.clone(),
                            received: received.clone(),
                        },
                        // TODO: fix this once we have the checked type's span
                        span: Span {
                            start: Position {
                                line: 0,
                                col: 0,
                                byte_offset: 0,
                            },
                            end: Position {
                                line: 0,
                                col: 0,
                                byte_offset: 0,
                            },
                        },
                    });
                }
            }
        }
        _ => {}
    }
}
