use std::{
    cell::RefCell,
    collections::HashSet,
    hash::{Hash, Hasher},
    rc::Rc,
};

use crate::ast::{
    base::base_declaration::EnumDecl,
    checked::checked_declaration::{CheckedFnType, CheckedStructDecl, CheckedTypeAliasDecl},
    DefinitionId, Span,
};

use super::checked_declaration::CheckedGenericParam;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeAssertion {
    pub target: DefinitionId,
    pub asserted_type: CheckedType,
}

#[derive(Clone, Debug)]
pub enum CheckedTypeKind {
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
    TypeAssertion(Box<TypeAssertion>),
    Array { item_type: Box<CheckedType>, size: usize },
    StructDecl(Rc<RefCell<CheckedStructDecl>>),
    GenericParam(CheckedGenericParam),
    EnumDecl(Rc<RefCell<EnumDecl>>),
    TypeAliasDecl(Rc<RefCell<CheckedTypeAliasDecl>>),
    FnType(CheckedFnType),
    Union(HashSet<CheckedType>),
    Unknown,
}

impl Eq for CheckedTypeKind {}
impl PartialEq for CheckedTypeKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CheckedTypeKind::Void, CheckedTypeKind::Void) => true,
            (CheckedTypeKind::Null, CheckedTypeKind::Null) => true,
            (CheckedTypeKind::Bool, CheckedTypeKind::Bool) => true,
            (CheckedTypeKind::U8, CheckedTypeKind::U8) => true,
            (CheckedTypeKind::U16, CheckedTypeKind::U16) => true,
            (CheckedTypeKind::U32, CheckedTypeKind::U32) => true,
            (CheckedTypeKind::U64, CheckedTypeKind::U64) => true,
            (CheckedTypeKind::USize, CheckedTypeKind::USize) => true,
            (CheckedTypeKind::ISize, CheckedTypeKind::ISize) => true,
            (CheckedTypeKind::I8, CheckedTypeKind::I8) => true,
            (CheckedTypeKind::I16, CheckedTypeKind::I16) => true,
            (CheckedTypeKind::I32, CheckedTypeKind::I32) => true,
            (CheckedTypeKind::I64, CheckedTypeKind::I64) => true,
            (CheckedTypeKind::F32, CheckedTypeKind::F32) => true,
            (CheckedTypeKind::F64, CheckedTypeKind::F64) => true,
            (CheckedTypeKind::Char, CheckedTypeKind::Char) => true,
            (CheckedTypeKind::Unknown, CheckedTypeKind::Unknown) => true,
            (CheckedTypeKind::GenericParam(a), CheckedTypeKind::GenericParam(b)) => a == b,
            (CheckedTypeKind::TypeAliasDecl(a), CheckedTypeKind::TypeAliasDecl(b)) => a == b,
            (CheckedTypeKind::StructDecl(a), CheckedTypeKind::StructDecl(b)) => a == b,
            (CheckedTypeKind::EnumDecl(a), CheckedTypeKind::EnumDecl(b)) => a == b,
            (CheckedTypeKind::FnType(a), CheckedTypeKind::FnType(b)) => a == b,
            (CheckedTypeKind::Union(a_items), CheckedTypeKind::Union(b_items)) => {
                if a_items.len() != b_items.len() {
                    return false;
                }
                // Order-insensitive comparison for unions
                a_items.iter().all(|item_a| b_items.contains(item_a)) && b_items.iter().all(|item_b| a_items.contains(item_b))
            }
            (
                CheckedTypeKind::Array {
                    item_type: ai,
                    size: asize,
                    ..
                },
                CheckedTypeKind::Array {
                    item_type: bi,
                    size: bsize,
                    ..
                },
            ) => ai == bi && asize == bsize,
            _ => false,
        }
    }
}

impl Hash for CheckedTypeKind {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);

        match self {
            CheckedTypeKind::Void => {}
            CheckedTypeKind::Null => {}
            CheckedTypeKind::Bool => {}
            CheckedTypeKind::U8 => {}
            CheckedTypeKind::U16 => {}
            CheckedTypeKind::U32 => {}
            CheckedTypeKind::U64 => {}
            CheckedTypeKind::USize => {}
            CheckedTypeKind::ISize => {}
            CheckedTypeKind::I8 => {}
            CheckedTypeKind::I16 => {}
            CheckedTypeKind::I32 => {}
            CheckedTypeKind::I64 => {}
            CheckedTypeKind::F32 => {}
            CheckedTypeKind::F64 => {}
            CheckedTypeKind::Char => {}
            CheckedTypeKind::Unknown => {}
            CheckedTypeKind::GenericParam(decl) => decl.hash(state),
            CheckedTypeKind::TypeAliasDecl(decl) => decl.borrow().hash(state),
            CheckedTypeKind::EnumDecl(decl) => decl.borrow().hash(state),
            CheckedTypeKind::StructDecl(decl) => decl.borrow().hash(state),
            CheckedTypeKind::FnType(decl) => decl.hash(state),
            CheckedTypeKind::Union(items) => {
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
            CheckedTypeKind::Array { item_type, size, .. } => {
                item_type.hash(state);
                size.hash(state);
            }
            CheckedTypeKind::TypeAssertion(assertion) => {
                assertion.target.hash(state);
                assertion.asserted_type.kind.hash(state);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CheckedType {
    pub kind: CheckedTypeKind,
    pub span: Span,
}

impl Eq for CheckedType {}
impl PartialEq for CheckedType {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}
impl Hash for CheckedType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
    }
}
