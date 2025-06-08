use std::rc::Rc;

use crate::{
    ast::checked::{
        checked_declaration::CheckedFnType,
        checked_type::{CheckedType, CheckedTypeKind},
    },
    check::SemanticChecker,
};

impl<'a> SemanticChecker<'a> {
    pub fn check_is_assignable(&mut self, source_type: &CheckedType, target_type: &CheckedType) -> bool {
        use CheckedTypeKind::*;
        // TODO: add recursion detection and handling

        match (&source_type.kind, &target_type.kind) {
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
            | (Null, Null)
            | (Void, Void)
            | (Unknown, _) => true,
            (Union(source), Union(target)) => source.iter().all(|source_item| {
                target
                    .iter()
                    .any(|target_item| self.check_is_assignable(source_item, target_item))
            }),
            (_, Union(target)) => target
                .iter()
                .any(|target_item| self.check_is_assignable(source_type, target_item)),

            (GenericParam(source), GenericParam(target)) => match (&source.constraint, &target.constraint) {
                (None, None) => true,
                (Some(_), None) => true,
                (None, Some(_)) => false,
                (Some(left_constraint), Some(right_constraint)) => self.check_is_assignable(left_constraint, right_constraint),
            },
            (source, GenericParam(target)) => match (&source, &target.constraint) {
                (_, None) => true,
                (_, Some(right_constraint)) => self.check_is_assignable(source_type, right_constraint),
            },
            (StructDecl(source), StructDecl(target)) => {
                if Rc::ptr_eq(source, target) {
                    return true;
                }

                let source = source.borrow();
                let target = target.borrow();

                if source.fields.len() != target.fields.len() {
                    return false;
                }

                let same_name = source.identifier.name == target.identifier.name;

                let assignable_fields = source.fields.iter().zip(target.fields.iter()).all(|(sp, tp)| {
                    let same_name = sp.identifier.name == tp.identifier.name;
                    let assignable = self.check_is_assignable(&sp.constraint, &tp.constraint);

                    same_name && assignable
                });

                same_name && assignable_fields
            }
            (EnumDecl(source), EnumDecl(target)) => source == target,
            (
                Array {
                    item_type: source_type,
                    size: source_size,
                    ..
                },
                Array {
                    item_type: target_type,
                    size: target_size,
                    ..
                },
            ) => {
                let same_size = source_size == target_size;
                let assignable_types = self.check_is_assignable(&source_type, &target_type);

                same_size && assignable_types
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

                let compatible_params = source_params
                    .iter()
                    .zip(target_params.iter())
                    .all(|(sp, tp)| self.check_is_assignable(&tp.constraint, &sp.constraint));

                let compatible_returns = self.check_is_assignable(source_return_type, target_return_type);

                compatible_params && compatible_returns
            }
            (TypeAliasDecl(source), _) => self.check_is_assignable(&source.borrow().value, target_type),
            (_, TypeAliasDecl(target)) => self.check_is_assignable(source_type, &target.borrow().value),
            _ => false,
        }
    }
}
