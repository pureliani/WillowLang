use std::{
    cell::RefCell,
    collections::HashSet,
    hash::{Hash, Hasher},
    rc::Rc,
};

use crate::ast::{
    checked::checked_declaration::{CheckedFnType, CheckedParam, CheckedTypeAliasDecl},
    Span,
};

use super::checked_declaration::CheckedGenericParam;

#[derive(Clone, Debug)]
pub enum TypeKind {
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
    Array { item_type: Box<Type>, size: usize },
    Struct(Vec<CheckedParam>),
    TypeAliasDecl(Rc<RefCell<CheckedTypeAliasDecl>>),
    GenericParam(CheckedGenericParam),
    FnType(CheckedFnType),
    Union(HashSet<Type>),
    Unknown,
    Pointer(Box<Type>),
}

impl Eq for TypeKind {}
impl PartialEq for TypeKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TypeKind::Void, TypeKind::Void) => true,
            (TypeKind::Null, TypeKind::Null) => true,
            (TypeKind::Bool, TypeKind::Bool) => true,
            (TypeKind::U8, TypeKind::U8) => true,
            (TypeKind::U16, TypeKind::U16) => true,
            (TypeKind::U32, TypeKind::U32) => true,
            (TypeKind::U64, TypeKind::U64) => true,
            (TypeKind::USize, TypeKind::USize) => true,
            (TypeKind::ISize, TypeKind::ISize) => true,
            (TypeKind::I8, TypeKind::I8) => true,
            (TypeKind::I16, TypeKind::I16) => true,
            (TypeKind::I32, TypeKind::I32) => true,
            (TypeKind::I64, TypeKind::I64) => true,
            (TypeKind::F32, TypeKind::F32) => true,
            (TypeKind::F64, TypeKind::F64) => true,
            (TypeKind::Char, TypeKind::Char) => true,
            (TypeKind::Unknown, TypeKind::Unknown) => true,
            (TypeKind::GenericParam(a), TypeKind::GenericParam(b)) => a == b,
            (TypeKind::TypeAliasDecl(a), TypeKind::TypeAliasDecl(b)) => a == b,
            (TypeKind::Struct(a), TypeKind::Struct(b)) => a == b,
            (TypeKind::FnType(a), TypeKind::FnType(b)) => a == b,
            (TypeKind::Pointer(a), TypeKind::Pointer(b)) => a == b,
            (TypeKind::Union(a_items), TypeKind::Union(b_items)) => {
                if a_items.len() != b_items.len() {
                    return false;
                }
                // Order-insensitive comparison for unions
                a_items.iter().all(|item_a| b_items.contains(item_a)) && b_items.iter().all(|item_b| a_items.contains(item_b))
            }
            (
                TypeKind::Array {
                    item_type: ai,
                    size: asize,
                    ..
                },
                TypeKind::Array {
                    item_type: bi,
                    size: bsize,
                    ..
                },
            ) => ai == bi && asize == bsize,
            _ => false,
        }
    }
}

impl Hash for TypeKind {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);

        match self {
            TypeKind::Void => {}
            TypeKind::Null => {}
            TypeKind::Bool => {}
            TypeKind::U8 => {}
            TypeKind::U16 => {}
            TypeKind::U32 => {}
            TypeKind::U64 => {}
            TypeKind::USize => {}
            TypeKind::ISize => {}
            TypeKind::I8 => {}
            TypeKind::I16 => {}
            TypeKind::I32 => {}
            TypeKind::I64 => {}
            TypeKind::F32 => {}
            TypeKind::F64 => {}
            TypeKind::Char => {}
            TypeKind::Unknown => {}
            TypeKind::Struct(fields) => fields.iter().for_each(|f| f.hash(state)),
            TypeKind::TypeAliasDecl(decl) => decl.borrow().hash(state),
            TypeKind::GenericParam(decl) => decl.hash(state),
            TypeKind::FnType(decl) => decl.hash(state),
            TypeKind::Pointer(inner) => inner.hash(state),
            TypeKind::Union(items) => {
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
                    item_hashes.sort_unstable();
                    for h in item_hashes {
                        h.hash(state);
                    }
                }
            }
            TypeKind::Array { item_type, size, .. } => {
                item_type.hash(state);
                size.hash(state);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Type {
    pub kind: TypeKind,
    pub span: Span,
}

impl Eq for Type {}
impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}
impl Hash for Type {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
    }
}
