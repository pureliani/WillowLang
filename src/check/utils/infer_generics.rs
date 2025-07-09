use crate::{
    ast::checked::{
        checked_declaration::CheckedFnType,
        checked_type::{Type, TypeKind},
    },
    check::{SemanticChecker, SemanticError},
};

use super::substitute_generics::GenericSubstitutionMap;

impl<'a> SemanticChecker<'a> {
    pub fn infer_generics(&mut self, expected: &Type, received: &Type, substitution: &mut GenericSubstitutionMap) {
        match (&expected.kind, &received.kind) {
            (TypeKind::GenericParam(expected_gp), _) => {
                let name = &expected_gp.identifier.name;
                if let Some(existing) = substitution.get(name) {
                    if existing != received {
                        self.errors.push(SemanticError::ConflictingGenericBinding {
                            generic_param: expected_gp.clone(),
                            existing: existing.clone(),
                            new: received.clone(),
                        });
                    }
                } else {
                    substitution.insert(name.clone(), received.clone());
                }
            }
            (
                TypeKind::Array {
                    item_type: expected_item_type,
                    ..
                },
                TypeKind::Array {
                    item_type: received_item_type,
                    ..
                },
            ) => {
                self.infer_generics(&expected_item_type, &received_item_type, substitution);
            }
            (TypeKind::Struct(expected_fields), TypeKind::Struct(received_fields)) => {
                for (expected_field, received_field) in expected_fields.iter().zip(received_fields.iter()) {
                    self.infer_generics(&expected_field.constraint, &received_field.constraint, substitution);
                }
            }
            (
                TypeKind::FnType(CheckedFnType {
                    params: generic_params,
                    return_type: generic_return_type,
                    generic_params: _,
                    span: _,
                    applied_type_args: _,
                }),
                TypeKind::FnType(CheckedFnType {
                    params: concrete_params,
                    return_type: concrete_return_type,
                    generic_params: _,
                    span: _,
                    applied_type_args: _,
                }),
            ) => {
                for (generic_param, concrete_param) in generic_params.iter().zip(concrete_params.iter()) {
                    self.infer_generics(&generic_param.constraint, &concrete_param.constraint, substitution);
                }

                self.infer_generics(&generic_return_type, &concrete_return_type, substitution);
            }
            (TypeKind::Union(expected_union), TypeKind::Union(received_union)) => {
                todo!()
            }
            (TypeKind::Union(expected_union), received) => {
                todo!()
            }
            _ => {}
        }
    }
}
