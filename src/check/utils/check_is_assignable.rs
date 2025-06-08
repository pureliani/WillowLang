use std::{collections::HashSet, rc::Rc};

use crate::{
    ast::checked::{
        checked_declaration::CheckedFnType,
        checked_type::{CheckedType, CheckedTypeKind},
    },
    check::SemanticChecker,
};

impl<'a> SemanticChecker<'a> {
    pub fn check_is_assignable(&mut self, source_type: &CheckedType, target_type: &CheckedType) -> bool {
        let mut currently_checking: HashSet<(usize, usize)> = HashSet::new();
        self.check_is_assignable_recursive(source_type, target_type, &mut currently_checking)
    }

    pub fn check_is_assignable_recursive(
        &mut self,
        source_type: &CheckedType,
        target_type: &CheckedType,
        currently_checking: &mut HashSet<(usize, usize)>,
    ) -> bool {
        use CheckedTypeKind::*;
        // TODO: add recursion detection and handling

        let key_opt: Option<(usize, usize)> = match (&source_type.kind, &target_type.kind) {
            (StructDecl(s_rc), StructDecl(t_rc)) => Some((Rc::as_ptr(s_rc) as usize, Rc::as_ptr(t_rc) as usize)),
            (TypeAliasDecl(s_rc), TypeAliasDecl(t_rc)) => Some((Rc::as_ptr(s_rc) as usize, Rc::as_ptr(t_rc) as usize)),
            _ => None,
        };

        if let Some(key) = key_opt {
            if currently_checking.contains(&key) {
                return true;
            }
            currently_checking.insert(key);
        }

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
            | (Null, Null)
            | (Void, Void)
            | (Unknown, _) => true,
            (Union(source), Union(target)) => source.iter().all(|source_item| {
                target
                    .iter()
                    .any(|target_item| self.check_is_assignable_recursive(source_item, target_item, currently_checking))
            }),
            (_, Union(target)) => target
                .iter()
                .any(|target_item| self.check_is_assignable_recursive(source_type, target_item, currently_checking)),
            (Union(source), _) => source
                .iter()
                .all(|source_item| self.check_is_assignable_recursive(source_item, target_type, currently_checking)),

            (GenericParam(source), GenericParam(target)) => match (&source.constraint, &target.constraint) {
                (None, None) => true,
                (Some(_), None) => true,
                (None, Some(_)) => false,
                (Some(left_constraint), Some(right_constraint)) => {
                    self.check_is_assignable_recursive(left_constraint, right_constraint, currently_checking)
                }
            },
            (source, GenericParam(target)) => match (&source, &target.constraint) {
                (_, None) => true,
                (_, Some(right_constraint)) => {
                    self.check_is_assignable_recursive(source_type, right_constraint, currently_checking)
                }
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
                    let assignable = self.check_is_assignable_recursive(&sp.constraint, &tp.constraint, currently_checking);

                    same_name && assignable
                });

                same_name && assignable_fields
            }
            (EnumDecl(source), EnumDecl(target)) => Rc::ptr_eq(source, target),
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
                let assignable_types = self.check_is_assignable_recursive(&source_type, &target_type, currently_checking);

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
                    .all(|(sp, tp)| self.check_is_assignable_recursive(&tp.constraint, &sp.constraint, currently_checking));

                let compatible_returns =
                    self.check_is_assignable_recursive(source_return_type, target_return_type, currently_checking);

                compatible_params && compatible_returns
            }
            (TypeAliasDecl(source), _) => {
                self.check_is_assignable_recursive(&source.borrow().value, target_type, currently_checking)
            }
            (_, TypeAliasDecl(target)) => {
                self.check_is_assignable_recursive(source_type, &target.borrow().value, currently_checking)
            }
            _ => false,
        };

        if let Some(key) = key_opt {
            currently_checking.remove(&key);
        }

        result
    }
}
