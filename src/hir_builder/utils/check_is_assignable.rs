use std::collections::HashSet;

use crate::hir_builder::{
    types::{
        checked_declaration::{CheckedFnType, CheckedTag},
        checked_type::{Type, TypeKind},
    },
    HIRBuilder,
};

impl<'a> HIRBuilder<'a> {
    pub fn check_is_assignable(&mut self, source_type: &Type, target_type: &Type) -> bool {
        let mut visited_declarations: HashSet<(usize, usize)> = HashSet::new();
        self.check_is_assignable_recursive(source_type, target_type, &mut visited_declarations)
    }

    pub fn check_is_tag_assignable(
        &mut self,
        source_tag: &CheckedTag,
        target_tag: &CheckedTag,
        visited_declarations: &mut HashSet<(usize, usize)>,
    ) -> bool {
        match (source_tag, target_tag) {
            (
                CheckedTag {
                    identifier: id_a,
                    value_type: Some(value_type_a),
                },
                CheckedTag {
                    identifier: id_b,
                    value_type: Some(value_type_b),
                },
            ) => {
                if id_a != id_b {
                    return false;
                }

                self.check_is_assignable_recursive(value_type_a, value_type_b, visited_declarations)
                    && self.check_is_assignable_recursive(value_type_b, value_type_a, visited_declarations)
            }
            (
                CheckedTag {
                    identifier: id_a,
                    value_type: None,
                },
                CheckedTag {
                    identifier: id_b,
                    value_type: None,
                },
            ) => id_a == id_b,
            _ => false,
        }
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
            | (String, String)
            | (Bool, Bool)
            | (Void, Void)
            | (Unknown, _) => true,
            (Union(source), Union(target)) => source.iter().all(|source_item| {
                target
                    .iter()
                    .any(|target_item| self.check_is_tag_assignable(source_item, target_item, visited_declarations))
            }),
            (Tag(source_item), Union(target_union)) => target_union
                .iter()
                .any(|target_item| self.check_is_tag_assignable(source_item, target_item, visited_declarations)),
            (Tag(t1), Tag(t2)) => self.check_is_tag_assignable(t1, t2, visited_declarations),
            (Pointer(source), Pointer(target)) => self.check_is_assignable_recursive(source, target, visited_declarations),
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
            (List(source_item_type), List(target_item_type)) => {
                self.check_is_assignable_recursive(source_item_type, target_item_type, visited_declarations)
                    && self.check_is_assignable_recursive(target_item_type, source_item_type, visited_declarations)
            }
            (
                FnType(CheckedFnType {
                    params: source_params,
                    return_type: source_return_type,
                    ..
                }),
                FnType(CheckedFnType {
                    params: target_params,
                    return_type: target_return_type,
                    ..
                }),
            ) => {
                if source_params.len() != target_params.len() {
                    return false;
                }

                let params_compatible = source_params
                    .iter()
                    .zip(target_params.iter())
                    .all(|(sp, tp)| self.check_is_assignable_recursive(&sp.constraint, &tp.constraint, visited_declarations));

                if !params_compatible {
                    return false;
                }

                let returns_compatible =
                    self.check_is_assignable_recursive(&source_return_type, &target_return_type, visited_declarations);

                returns_compatible
            }
            (TypeAliasDecl(source), _) => self.check_is_assignable_recursive(&source.value, target_type, visited_declarations),
            (_, TypeAliasDecl(target)) => self.check_is_assignable_recursive(source_type, &target.value, visited_declarations),
            _ => false,
        };

        result
    }
}
