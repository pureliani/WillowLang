use std::collections::HashSet;

use crate::hir::{
    types::{
        checked_declaration::{FnType, TagType},
        checked_type::{PointerKind, StructKind, Type},
    },
    FunctionBuilder,
};

impl FunctionBuilder {
    pub fn check_is_tag_assignable(
        &self,
        source_tag: &TagType,
        target_tag: &TagType,
        visited_declarations: &mut HashSet<(usize, usize)>,
    ) -> bool {
        match (source_tag, target_tag) {
            (
                TagType {
                    id: id_a,
                    value_type: Some(value_type_a),
                    ..
                },
                TagType {
                    id: id_b,
                    value_type: Some(value_type_b),
                    ..
                },
            ) => {
                if id_a != id_b {
                    return false;
                }

                self.check_is_assignable_recursive(
                    value_type_a,
                    value_type_b,
                    visited_declarations,
                ) && self.check_is_assignable_recursive(
                    value_type_b,
                    value_type_a,
                    visited_declarations,
                )
            }
            (
                TagType {
                    id: id_a,
                    value_type: None,
                    ..
                },
                TagType {
                    id: id_b,
                    value_type: None,
                    ..
                },
            ) => id_a == id_b,
            _ => false,
        }
    }

    pub fn check_is_assignable(&self, source_type: &Type, target_type: &Type) -> bool {
        let mut visited_declarations: HashSet<(usize, usize)> = HashSet::new();
        self.check_is_assignable_recursive(
            source_type,
            target_type,
            &mut visited_declarations,
        )
    }

    pub fn check_is_assignable_recursive(
        &self,
        source_type: &Type,
        target_type: &Type,
        visited_declarations: &mut HashSet<(usize, usize)>,
    ) -> bool {
        use Type::*;
        // TODO: add recursion detection and handling

        let result = match (&source_type, &target_type) {
            (I8, I8)
            | (I16, I16)
            | (I32, I32)
            | (I64, I64)
            | (ISize, ISize)
            | (USize, USize)
            | (U8, U8)
            | (U16, U16)
            | (U32, U32)
            | (U64, U64)
            | (F32, F32)
            | (F64, F64)
            | (Bool, Bool)
            | (Void, Void)
            | (_, Void)
            | (_, Unknown)
            | (Unknown, _) => true,

            (
                Pointer {
                    kind: kind_a,
                    to: to_a,
                },
                Pointer {
                    kind: kind_b,
                    to: to_b,
                },
            ) => {
                let kind_compatible = match (kind_a, kind_b) {
                    (PointerKind::Raw, PointerKind::Raw) => true,
                    (PointerKind::Ref, PointerKind::Ref) => true,
                    (PointerKind::Mut, PointerKind::Mut) => true,
                    // Mut can be assigned to Ref (downgrade)
                    (PointerKind::Mut, PointerKind::Ref) => true,
                    _ => false,
                };

                if !kind_compatible {
                    return false;
                }

                self.check_is_assignable_recursive(to_a, to_b, visited_declarations)
            }

            (Struct(source), Struct(target)) => match (source, target) {
                (
                    StructKind::Union { variants: source },
                    StructKind::Union { variants: target },
                ) => source.into_iter().all(|source_item| {
                    target.into_iter().any(|target_item| {
                        self.check_is_tag_assignable(
                            source_item,
                            target_item,
                            visited_declarations,
                        )
                    })
                }),
                (
                    StructKind::Tag(source_item),
                    StructKind::Union {
                        variants: target_union,
                    },
                ) => target_union.iter().any(|target_item| {
                    self.check_is_tag_assignable(
                        source_item,
                        target_item,
                        visited_declarations,
                    )
                }),
                (StructKind::Tag(t1), StructKind::Tag(t2)) => {
                    self.check_is_tag_assignable(t1, t2, visited_declarations)
                }
                (
                    StructKind::UserDefined(source_fields),
                    StructKind::UserDefined(target_fields),
                ) => {
                    if source_fields.len() != target_fields.len() {
                        return false;
                    }

                    let is_assignable = source_fields
                        .iter()
                        .zip(target_fields.iter())
                        .all(|(sp, tp)| {
                            let same_name = sp.identifier.name == tp.identifier.name;
                            let assignable = self.check_is_assignable_recursive(
                                &sp.ty,
                                &tp.ty,
                                visited_declarations,
                            );

                            same_name && assignable
                        });

                    is_assignable
                }
                (
                    StructKind::ClosureObject(source_object),
                    StructKind::ClosureObject(target_object),
                ) => todo!(),
                (
                    StructKind::ClosureEnv(source_env),
                    StructKind::ClosureEnv(target_env),
                ) => todo!(),
                (
                    StructKind::List(source_item_type),
                    StructKind::List(target_item_type),
                ) => self.check_is_assignable_recursive(
                    source_item_type,
                    target_item_type,
                    visited_declarations,
                ),
                (StructKind::String, StructKind::String) => true,
                _ => false,
            },
            (
                Fn(FnType {
                    params: source_params,
                    return_type: source_return_type,
                    ..
                }),
                Fn(FnType {
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
                    .all(|(sp, tp)| {
                        self.check_is_assignable_recursive(
                            &sp.ty,
                            &tp.ty,
                            visited_declarations,
                        )
                    });

                if !params_compatible {
                    return false;
                }

                let returns_compatible = self.check_is_assignable_recursive(
                    &source_return_type,
                    &target_return_type,
                    visited_declarations,
                );

                returns_compatible
            }
            _ => false,
        };

        result
    }
}
