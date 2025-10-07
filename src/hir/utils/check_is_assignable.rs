use std::collections::HashSet;

use crate::hir::{
    types::{
        checked_declaration::CheckedFnType,
        checked_type::{Type, TypeKind},
    },
    FunctionBuilder,
};

impl FunctionBuilder {
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
            (Pointer(source), Pointer(target)) => {
                self.check_is_assignable_recursive(source, target, visited_declarations)
            }
            (Struct(source_decl), Struct(target_decl)) => {
                todo!()
            }
            (Enum(source_decl), Enum(target_decl)) => {
                todo!()
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
                    .all(|(sp, tp)| {
                        self.check_is_assignable_recursive(
                            &sp.constraint,
                            &tp.constraint,
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
            (TypeAliasDecl(source), _) => self.check_is_assignable_recursive(
                &source.value,
                target_type,
                visited_declarations,
            ),
            (_, TypeAliasDecl(target)) => self.check_is_assignable_recursive(
                source_type,
                &target.value,
                visited_declarations,
            ),
            _ => false,
        };

        result
    }
}
