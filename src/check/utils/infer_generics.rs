use crate::{
    ast::{checked::checked_type::CheckedType, Position, Span},
    check::{SemanticChecker, SemanticError, SemanticErrorKind},
};

use super::substitute_generics::GenericSubstitutionMap;

impl<'a> SemanticChecker<'a> {
    pub fn infer_generics(
        &mut self,
        expected: &CheckedType,
        received: &CheckedType,
        substitution: &mut GenericSubstitutionMap,
    ) {
        match (expected, received) {
            (CheckedType::GenericParam(expected_generic_param), received_kind) => {
                let name = &expected_generic_param.identifier.name;
                if let Some(existing) = substitution.get(name) {
                    if existing != received_kind {
                        self.errors.push(SemanticError {
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
                self.infer_generics(expected_item_type, received_item_type, substitution);
            }
            (CheckedType::StructDecl(expected), CheckedType::StructDecl(received)) => {
                for (generic_prop, concrete_prop) in
                    expected.properties.iter().zip(received.properties.iter())
                {
                    self.infer_generics(
                        &generic_prop.constraint,
                        &concrete_prop.constraint,
                        substitution,
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
                for (generic_param, concrete_param) in
                    generic_params.iter().zip(concrete_params.iter())
                {
                    self.infer_generics(
                        &generic_param.constraint,
                        &concrete_param.constraint,
                        substitution,
                    );
                }

                self.infer_generics(&generic_return_type, &concrete_return_type, substitution);
            }
            (CheckedType::Union(expected_union), CheckedType::Union(received_union)) => {
                todo!()
            }
            (CheckedType::Union(expected_union), received) => {
                todo!()
            }
            _ => {}
        }
    }
}
