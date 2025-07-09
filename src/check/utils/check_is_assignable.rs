use std::collections::HashSet;

use crate::{
    ast::checked::{
        checked_declaration::CheckedFnType,
        checked_type::{Type, TypeKind},
    },
    check::{utils::substitute_generics::GenericSubstitutionMap, SemanticChecker},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_is_assignable(&mut self, source_type: &Type, target_type: &Type) -> bool {
        let mut visited_declarations: HashSet<(usize, usize)> = HashSet::new();
        self.check_is_assignable_recursive(source_type, target_type, &mut visited_declarations)
    }

    pub fn check_is_assignable_recursive(
        &mut self,
        source_type: &Type,
        target_type: &Type,
        visited_declarations: &mut HashSet<(usize, usize)>,
    ) -> bool {
        use TypeKind::*;
        // TODO: add recursion detection and handling

        let result = match (&source_type.kind, &target_type.kind) {
            (I8, I8)
            | (I16, I16)
            | (I32, I32)
            | (I64, I64)
            | (ISize, ISize)
            | (U8, U8)
            | (U16, U16)
            | (U32, U32)
            | (U64, U64)
            | (USize, USize)
            | (F32, F32)
            | (F64, F64)
            | (Char, Char)
            | (Bool, Bool)
            | (Void, Void)
            | (Null, Null)
            | (Unknown, _) => true,
            (Union(source), Union(target)) => source.iter().all(|source_item| {
                target
                    .iter()
                    .any(|target_item| self.check_is_assignable_recursive(source_item, target_item, visited_declarations))
            }),
            (_, Union(target)) => target
                .iter()
                .any(|target_item| self.check_is_assignable_recursive(source_type, target_item, visited_declarations)),
            (Union(source), _) => source
                .iter()
                .all(|source_item| self.check_is_assignable_recursive(source_item, target_type, visited_declarations)),

            (GenericParam(source), GenericParam(target)) => match (&source.constraint, &target.constraint) {
                (None, None) => true,
                (Some(_), None) => true,
                (None, Some(_)) => false,
                (Some(left_constraint), Some(right_constraint)) => {
                    self.check_is_assignable_recursive(left_constraint, right_constraint, visited_declarations)
                }
            },
            (GenericParam(source), target) => match (&source.constraint, target) {
                (None, _) => false,
                (Some(generic_constraint), _) => {
                    self.check_is_assignable_recursive(generic_constraint, target_type, visited_declarations)
                }
            },
            (source, GenericParam(target)) => match (source, &target.constraint) {
                (_, None) => true,
                (_, Some(generic_constraint)) => {
                    self.check_is_assignable_recursive(source_type, generic_constraint, visited_declarations)
                }
            },
            (Struct(source_fields), Struct(target_fields)) => {
                if source_fields.len() != target_fields.len() {
                    return false;
                }

                let is_assignable = source_fields.iter().zip(target_fields.iter()).all(|(sp, tp)| {
                    let same_name = sp.identifier.name == tp.identifier.name;
                    let assignable = self.check_is_assignable_recursive(&sp.constraint, &tp.constraint, visited_declarations);

                    same_name && assignable
                });

                is_assignable
            }
            (
                Array {
                    item_type: source_item_type,
                    size: source_size,
                    ..
                },
                Array {
                    item_type: target_item_type,
                    size: target_size,
                    ..
                },
            ) => {
                source_size == target_size
                    && (self.check_is_assignable_recursive(source_item_type, target_item_type, visited_declarations)
                        && self.check_is_assignable_recursive(target_item_type, source_item_type, visited_declarations))
            }
            (
                FnType(CheckedFnType {
                    params: source_params,
                    return_type: source_return_type,
                    generic_params: source_generic_params,
                    ..
                }),
                FnType(CheckedFnType {
                    params: target_params,
                    return_type: target_return_type,
                    generic_params: target_generic_params,
                    ..
                }),
            ) => {
                if source_generic_params.len() != target_generic_params.len() {
                    return false;
                }
                if source_params.len() != target_params.len() {
                    return false;
                }

                let fn_generics_constraints_compatible =
                    source_generic_params
                        .iter()
                        .zip(target_generic_params.iter())
                        .all(|(sgp, tgp)| match (&sgp.constraint, &tgp.constraint) {
                            (Some(sc), Some(tc)) => self.check_is_assignable_recursive(sc, tc, visited_declarations),
                            (Some(_), None) => true,
                            (None, Some(_)) => false,
                            (None, None) => true,
                        });

                if !fn_generics_constraints_compatible {
                    return false;
                }

                let mut substitution_map = GenericSubstitutionMap::new();
                for (sgp, tgp) in source_generic_params.iter().zip(target_generic_params.iter()) {
                    let source_generic_param_as_type = Type {
                        kind: GenericParam(sgp.clone()),
                        span: sgp.identifier.span,
                    };
                    substitution_map.insert(tgp.identifier.name, source_generic_param_as_type);
                }

                let params_compatible = source_params.iter().zip(target_params.iter()).all(|(sp, tp)| {
                    let target_param_type_substituted = self.substitute_generics(&tp.constraint, &substitution_map);
                    self.check_is_assignable_recursive(&target_param_type_substituted, &sp.constraint, visited_declarations)
                });

                if !params_compatible {
                    return false;
                }

                let target_return_type_substituted = self.substitute_generics(&target_return_type, &substitution_map);
                let returns_compatible = self.check_is_assignable_recursive(
                    &source_return_type,
                    &target_return_type_substituted,
                    visited_declarations,
                );

                returns_compatible
            }
            (TypeAliasDecl(source), _) => {
                self.check_is_assignable_recursive(&source.borrow().value, target_type, visited_declarations)
            }
            (_, TypeAliasDecl(target)) => {
                self.check_is_assignable_recursive(source_type, &target.borrow().value, visited_declarations)
            }
            _ => false,
        };

        result
    }
}
