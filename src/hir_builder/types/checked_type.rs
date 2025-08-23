use std::hash::{Hash, Hasher};

use crate::{
    ast::Span,
    hir_builder::types::checked_declaration::{CheckedFnType, CheckedParam, CheckedTag, CheckedTypeAliasDecl},
};

#[derive(Clone, Debug)]
pub enum TypeKind {
    Void,
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
    String,
    Union(Vec<CheckedTag>),
    Tag(CheckedTag),
    List(Box<Type>),
    Struct(Vec<CheckedParam>),
    TypeAliasDecl(CheckedTypeAliasDecl),
    FnType(CheckedFnType),
    Pointer(Box<Type>),
    Unknown,
}

impl Eq for TypeKind {}
impl PartialEq for TypeKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TypeKind::Void, TypeKind::Void) => true,
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
            (TypeKind::String, TypeKind::String) => true,
            (TypeKind::Unknown, TypeKind::Unknown) => true,
            (TypeKind::TypeAliasDecl(a), TypeKind::TypeAliasDecl(b)) => a == b,
            (
                TypeKind::Tag(CheckedTag {
                    identifier: id_a,
                    value_type: kind_a,
                }),
                TypeKind::Tag(CheckedTag {
                    identifier: id_b,
                    value_type: kind_b,
                }),
            ) => id_a == id_b && kind_a == kind_b,
            (TypeKind::Struct(a), TypeKind::Struct(b)) => a == b,
            (TypeKind::FnType(a), TypeKind::FnType(b)) => a == b,
            (TypeKind::Pointer(a), TypeKind::Pointer(b)) => a == b,
            (TypeKind::List(t1), TypeKind::List(t2)) => t1 == t2,
            (TypeKind::Union(u1), TypeKind::Union(u2)) => {
                if u1.len() != u2.len() {
                    return false;
                }

                u1.iter().all(|u1_element| u2.contains(u1_element)) && u2.iter().all(|u2_element| u1.contains(u2_element))
            }
            _ => false,
        }
    }
}

impl Hash for TypeKind {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);

        match self {
            TypeKind::Void => {}
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
            TypeKind::String => {}
            TypeKind::Unknown => {}
            TypeKind::Struct(fields) => fields.iter().for_each(|f| f.hash(state)),
            TypeKind::TypeAliasDecl(decl) => decl.hash(state),
            TypeKind::FnType(decl) => decl.hash(state),
            TypeKind::Pointer(inner) => inner.hash(state),
            TypeKind::List(item_type) => {
                item_type.hash(state);
            }
            TypeKind::Tag(CheckedTag { identifier, value_type }) => {
                identifier.hash(state);
                value_type.hash(state);
            }
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
