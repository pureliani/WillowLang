use std::hash::{Hash, Hasher};

use crate::ast::{base::base_declaration::EnumDecl, Span};

use super::checked_declaration::{
    CheckedGenericParam, CheckedParam, GenericStructDecl, GenericTypeAliasDecl, StructDecl,
    TypeAliasDecl,
};

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
    GenericStructDecl(GenericStructDecl),
    StructDecl(StructDecl),
    Enum(EnumDecl),
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
    GenericTypeAliasDecl(GenericTypeAliasDecl),
    TypeAliasDecl(TypeAliasDecl),
    // Infix types
    Union(Vec<CheckedType>),
    // Suffix types
    Array {
        item_type: Box<CheckedType>,
        size: usize,
    },
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
            (CheckedTypeKind::GenericStructDecl(a), CheckedTypeKind::GenericStructDecl(b)) => {
                a == b
            }
            (CheckedTypeKind::StructDecl(a), CheckedTypeKind::StructDecl(b)) => a == b,
            (CheckedTypeKind::Enum(a), CheckedTypeKind::Enum(b)) => a == b,
            (CheckedTypeKind::GenericParam(a), CheckedTypeKind::GenericParam(b)) => a == b,
            (
                CheckedTypeKind::GenericFnType {
                    params: ap,
                    return_type: ar,
                    generic_params: agp,
                },
                CheckedTypeKind::GenericFnType {
                    params: bp,
                    return_type: br,
                    generic_params: bgp,
                },
            ) => ap == bp && ar == br && agp == bgp,
            (
                CheckedTypeKind::FnType {
                    params: ap,
                    return_type: ar,
                },
                CheckedTypeKind::FnType {
                    params: bp,
                    return_type: br,
                },
            ) => ap == bp && ar == br,
            (
                CheckedTypeKind::GenericTypeAliasDecl(a),
                CheckedTypeKind::GenericTypeAliasDecl(b),
            ) => a == b,
            (CheckedTypeKind::TypeAliasDecl(a), CheckedTypeKind::TypeAliasDecl(b)) => a == b,
            (CheckedTypeKind::Union(a_items), CheckedTypeKind::Union(b_items)) => {
                if a_items.len() != b_items.len() {
                    return false;
                }
                // Order-insensitive comparison for unions
                a_items.iter().all(|item_a| b_items.contains(item_a))
                    && b_items.iter().all(|item_b| a_items.contains(item_b))
            }
            (
                CheckedTypeKind::Array {
                    item_type: ai,
                    size: asize,
                },
                CheckedTypeKind::Array {
                    item_type: bi,
                    size: bsize,
                },
            ) => ai == bi && asize == bsize,
            (CheckedTypeKind::Unknown, CheckedTypeKind::Unknown) => true,
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
            CheckedTypeKind::GenericStructDecl(gsd) => gsd.hash(state),
            CheckedTypeKind::StructDecl(sd) => sd.hash(state),
            CheckedTypeKind::Enum(e) => e.hash(state),
            CheckedTypeKind::GenericParam(gp) => gp.hash(state),
            CheckedTypeKind::GenericFnType {
                params,
                return_type,
                generic_params,
            } => {
                params.hash(state);
                return_type.hash(state);
                generic_params.hash(state);
            }
            CheckedTypeKind::FnType {
                params,
                return_type,
            } => {
                params.hash(state);
                return_type.hash(state);
            }
            CheckedTypeKind::GenericTypeAliasDecl(gta) => gta.hash(state),
            CheckedTypeKind::TypeAliasDecl(ta) => ta.hash(state),
            CheckedTypeKind::Union(items) => {
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
            CheckedTypeKind::Array { item_type, size } => {
                item_type.hash(state);
                size.hash(state);
            }
            CheckedTypeKind::Unknown => {}
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TypeSpan {
    Expr(Span),
    Annotation(Span),
    Decl(Span),
    None,
}

#[derive(Clone, Debug)]
pub struct CheckedType {
    pub kind: CheckedTypeKind,
    pub span: TypeSpan,
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

impl CheckedType {
    pub fn unwrap_decl_span(&self) -> Span {
        match self.span {
            TypeSpan::Decl(s) => s,
            _ => {
                panic!(
                    "Expected the type of span to be TypeSpan::Decl on {:#?}",
                    self
                )
            }
        }
    }

    pub fn unwrap_expr_span(&self) -> Span {
        match self.span {
            TypeSpan::Expr(s) => s,
            _ => {
                panic!(
                    "Expected the type of span to be TypeSpan::Expr on {:#?}",
                    self
                )
            }
        }
    }

    pub fn unwrap_annotation_span(&self) -> Span {
        match self.span {
            TypeSpan::Annotation(s) => s,
            _ => {
                panic!(
                    "Expected the type of span to be TypeSpan::Annotation on {:#?}",
                    self
                )
            }
        }
    }
}
