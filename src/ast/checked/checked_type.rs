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

impl CheckedType {
    pub fn to_string(&self) -> String {
        match self {
            CheckedType::Void => "void".to_owned(),
            CheckedType::Null => "null".to_owned(),
            CheckedType::Bool => "bool".to_owned(),
            CheckedType::U8 => "u8".to_owned(),
            CheckedType::U16 => "u16".to_owned(),
            CheckedType::U32 => "u32".to_owned(),
            CheckedType::U64 => "u64".to_owned(),
            CheckedType::USize => "usize".to_owned(),
            CheckedType::ISize => "isize".to_owned(),
            CheckedType::I8 => "i8".to_owned(),
            CheckedType::I16 => "i16".to_owned(),
            CheckedType::I32 => "i32".to_owned(),
            CheckedType::I64 => "i64".to_owned(),
            CheckedType::F32 => "f32".to_owned(),
            CheckedType::F64 => "f64".to_owned(),
            CheckedType::Char => "char".to_owned(),
            CheckedType::Unknown => "unknown".to_owned(),
            CheckedType::GenericStructDecl(checked_generic_struct_decl) => {
                checked_generic_struct_decl.to_string()
            }
            CheckedType::StructDecl(checked_struct_decl) => checked_struct_decl.to_string(),
            CheckedType::EnumDecl(enum_decl) => enum_decl.to_string(),
            CheckedType::GenericParam(checked_generic_param) => checked_generic_param.to_string(),
            CheckedType::GenericFnType {
                params,
                return_type,
                generic_params,
            } => {
                let generic_params_str = if !generic_params.is_empty() {
                    let joined = generic_params
                        .iter()
                        .map(|gp| gp.to_string())
                        .collect::<Vec<String>>()
                        .join(", ");

                    format!("<{}>", joined)
                } else {
                    "".to_owned()
                };

                let params_str = {
                    let joined = params
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<String>>()
                        .join(",\n");

                    format!("({})", joined)
                };

                format!(
                    "({}{} => {})",
                    generic_params_str,
                    params_str,
                    return_type.to_string()
                )
            }
            CheckedType::FnType {
                params,
                return_type,
            } => {
                let params_str = {
                    let joined = params
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<String>>()
                        .join(",\n");

                    format!("({})", joined)
                };

                format!("({} => {})", params_str, return_type.to_string())
            }
            CheckedType::GenericTypeAliasDecl(checked_generic_type_alias_decl) => {
                checked_generic_type_alias_decl.to_string()
            }
            CheckedType::TypeAliasDecl(checked_type_alias_decl) => {
                checked_type_alias_decl.to_string()
            }
            CheckedType::Union(hash_set) => hash_set
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<String>>()
                .join(" | "),

            CheckedType::Array { item_type, size } => {
                format!("({})[{}]", item_type.to_string(), size)
            }
        }
    }
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
