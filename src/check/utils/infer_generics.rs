use crate::{
    ast::checked::{
        checked_declaration::CheckedFnType,
        checked_type::{CheckedType, CheckedTypeKind},
    },
    check::{SemanticChecker, SemanticError},
};

use super::substitute_generics::GenericSubstitutionMap;

impl<'a> SemanticChecker<'a> {
    pub fn infer_generics(&mut self, expected: &CheckedType, received: &CheckedType, substitution: &mut GenericSubstitutionMap) {
        match (&expected.kind, &received.kind) {
            (CheckedTypeKind::GenericParam(expected_gp), _) => {
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
                CheckedTypeKind::Array {
                    item_type: expected_item_type,
                    ..
                },
                CheckedTypeKind::Array {
                    item_type: received_item_type,
                    ..
                },
            ) => {
                self.infer_generics(&expected_item_type, &received_item_type, substitution);
            }
            (CheckedTypeKind::StructDecl(expected), CheckedTypeKind::StructDecl(received)) => {
                let expected = expected.borrow();
                let received = received.borrow();

                for (expected_field, received_field) in expected.fields.iter().zip(received.fields.iter()) {
                    self.infer_generics(&expected_field.constraint, &received_field.constraint, substitution);
                }
            }
            (
                CheckedTypeKind::FnType(CheckedFnType {
                    params: generic_params,
                    return_type: generic_return_type,
                    generic_params: _,
                    span: _,
                    applied_type_args: _,
                }),
                CheckedTypeKind::FnType(CheckedFnType {
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
            (CheckedTypeKind::Union(expected_union), CheckedTypeKind::Union(received_union)) => {
                todo!()
            }
            (CheckedTypeKind::Union(expected_union), received) => {
                todo!()
            }
            _ => {}
        }
    }
}
