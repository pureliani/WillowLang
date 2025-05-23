use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
};

use crate::ast::base::base_declaration::EnumDecl;

use super::checked_declaration::{
    CheckedGenericParam, CheckedGenericStructDecl, CheckedGenericTypeAliasDecl, CheckedParam,
    CheckedStructDecl, CheckedTypeAliasDecl,
};

#[derive(Clone, Debug)]
pub enum CheckedType {
    Void,
    Null,
    Bool,
    U8,
    U16,
    U32,
    U64,
    USize,
    ISize,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Char,
    GenericStructDecl(CheckedGenericStructDecl),
    StructDecl(CheckedStructDecl),
    EnumDecl(EnumDecl),
    GenericParam(CheckedGenericParam),
    GenericFnType {
        params: Vec<CheckedParam>,
        return_type: Box<CheckedType>,
        generic_params: Vec<CheckedGenericParam>,
    },
    FnType {
        params: Vec<CheckedParam>,
        return_type: Box<CheckedType>,
    },
    GenericTypeAliasDecl(CheckedGenericTypeAliasDecl),
    TypeAliasDecl(CheckedTypeAliasDecl),
    // Infix types
    Union(HashSet<CheckedType>),
    // Suffix types
    Array {
        item_type: Box<CheckedType>,
        size: usize,
    },
    Unknown,
}

impl Eq for CheckedType {}
impl PartialEq for CheckedType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CheckedType::Void, CheckedType::Void) => true,
            (CheckedType::Null, CheckedType::Null) => true,
            (CheckedType::Bool, CheckedType::Bool) => true,
            (CheckedType::U8, CheckedType::U8) => true,
            (CheckedType::U16, CheckedType::U16) => true,
            (CheckedType::U32, CheckedType::U32) => true,
            (CheckedType::U64, CheckedType::U64) => true,
            (CheckedType::USize, CheckedType::USize) => true,
            (CheckedType::ISize, CheckedType::ISize) => true,
            (CheckedType::I8, CheckedType::I8) => true,
            (CheckedType::I16, CheckedType::I16) => true,
            (CheckedType::I32, CheckedType::I32) => true,
            (CheckedType::I64, CheckedType::I64) => true,
            (CheckedType::F32, CheckedType::F32) => true,
            (CheckedType::F64, CheckedType::F64) => true,
            (CheckedType::Char, CheckedType::Char) => true,
            (CheckedType::GenericStructDecl(a), CheckedType::GenericStructDecl(b)) => a == b,
            (CheckedType::StructDecl(a), CheckedType::StructDecl(b)) => a == b,
            (CheckedType::EnumDecl(a), CheckedType::EnumDecl(b)) => a == b,
            (CheckedType::GenericParam(a), CheckedType::GenericParam(b)) => a == b,
            (
                CheckedType::GenericFnType {
                    params: ap,
                    return_type: ar,
                    generic_params: agp,
                },
                CheckedType::GenericFnType {
                    params: bp,
                    return_type: br,
                    generic_params: bgp,
                },
            ) => ap == bp && ar == br && agp == bgp,
            (
                CheckedType::FnType {
                    params: ap,
                    return_type: ar,
                },
                CheckedType::FnType {
                    params: bp,
                    return_type: br,
                },
            ) => ap == bp && ar == br,
            (CheckedType::GenericTypeAliasDecl(a), CheckedType::GenericTypeAliasDecl(b)) => a == b,
            (CheckedType::TypeAliasDecl(a), CheckedType::TypeAliasDecl(b)) => a == b,
            (CheckedType::Union(a_items), CheckedType::Union(b_items)) => {
                if a_items.len() != b_items.len() {
                    return false;
                }
                // Order-insensitive comparison for unions
                a_items.iter().all(|item_a| b_items.contains(item_a))
                    && b_items.iter().all(|item_b| a_items.contains(item_b))
            }
            (
                CheckedType::Array {
                    item_type: ai,
                    size: asize,
                },
                CheckedType::Array {
                    item_type: bi,
                    size: bsize,
                },
            ) => ai == bi && asize == bsize,
            (CheckedType::Unknown, CheckedType::Unknown) => true,
            _ => false,
        }
    }
}

impl Hash for CheckedType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);

        match self {
            CheckedType::Void => {}
            CheckedType::Null => {}
            CheckedType::Bool => {}
            CheckedType::U8 => {}
            CheckedType::U16 => {}
            CheckedType::U32 => {}
            CheckedType::U64 => {}
            CheckedType::USize => {}
            CheckedType::ISize => {}
            CheckedType::I8 => {}
            CheckedType::I16 => {}
            CheckedType::I32 => {}
            CheckedType::I64 => {}
            CheckedType::F32 => {}
            CheckedType::F64 => {}
            CheckedType::Char => {}
            CheckedType::GenericStructDecl(gsd) => gsd.hash(state),
            CheckedType::StructDecl(sd) => sd.hash(state),
            CheckedType::EnumDecl(e) => e.hash(state),
            CheckedType::GenericParam(gp) => gp.hash(state),
            CheckedType::GenericFnType {
                params,
                return_type,
                generic_params,
            } => {
                params.hash(state);
                return_type.hash(state);
                generic_params.hash(state);
            }
            CheckedType::FnType {
                params,
                return_type,
            } => {
                params.hash(state);
                return_type.hash(state);
            }
            CheckedType::GenericTypeAliasDecl(gta) => gta.hash(state),
            CheckedType::TypeAliasDecl(ta) => ta.hash(state),
            CheckedType::Union(items) => {
                // For order-insensitive hashing of unions:
                // 1. Hash the length.
                // 2. Hash each item's hash XORed together (or summed, but XOR is common).
                //    This makes the order not matter.
                // A more robust way is to sort a temporary list of hashes.
                state.write_usize(items.len());
                if !items.is_empty() {
                    let mut item_hashes: Vec<u64> = items
                        .iter()
                        .map(|item| {
                            let mut item_hasher = std::collections::hash_map::DefaultHasher::new();
                            item.hash(&mut item_hasher);
                            item_hasher.finish()
                        })
                        .collect();
                    item_hashes.sort_unstable(); // Sort hashes for canonical order
                    for h in item_hashes {
                        h.hash(state);
                    }
                }
            }
            CheckedType::Array { item_type, size } => {
                item_type.hash(state);
                size.hash(state);
            }
            CheckedType::Unknown => {}
        }
    }
}
