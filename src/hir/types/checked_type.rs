use std::hash::{Hash, Hasher};

use crate::{
    ast::{IdentifierNode, Span},
    hir::types::checked_declaration::{
        CheckedFnType, CheckedTagType, CheckedTypeAliasDecl,
    },
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
    List(Box<Type>),
    Tag(CheckedTagType),
    Union(Vec<CheckedTagType>),
    Struct(Vec<(IdentifierNode, Type)>),
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
            (TypeKind::Struct(a), TypeKind::Struct(b)) => a == b,
            (TypeKind::FnType(a), TypeKind::FnType(b)) => a == b,
            (TypeKind::Tag(t1), TypeKind::Tag(t2)) => {
                t1.identifier == t2.identifier && t1.value == t2.value
            }
            (TypeKind::Pointer(type_a), TypeKind::Pointer(type_b)) => {
                type_a.kind == type_b.kind
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
            TypeKind::Struct(decl) => decl.hash(state),
            TypeKind::TypeAliasDecl(decl) => decl.hash(state),
            TypeKind::FnType(decl) => decl.hash(state),
            TypeKind::Tag(tag) => tag.hash(state),
            TypeKind::Pointer(t) => t.hash(state),
            TypeKind::List(_) => todo!(),
            TypeKind::Union(checked_tag_types) => todo!(),
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
