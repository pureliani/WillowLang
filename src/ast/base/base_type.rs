use crate::{
    ast::{IdentifierNode, Span},
    tokenize::NumberKind,
};

use super::base_declaration::{GenericParam, Param};

#[derive(Clone, Debug, PartialEq)]
pub enum TypeAnnotationKind {
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
    Char,
    Identifier(IdentifierNode),
    Struct(Vec<Param>),
    Array {
        item_type: Box<TypeAnnotation>,
        size: NumberKind,
    },
    FnType {
        params: Vec<Param>,
        return_type: Box<TypeAnnotation>,
        generic_params: Vec<GenericParam>,
    },
    // Infix types
    Union(Vec<TypeAnnotation>),
    // Suffix types
    GenericApply {
        left: Box<TypeAnnotation>,
        args: Vec<TypeAnnotation>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeAnnotation {
    pub kind: TypeAnnotationKind,
    pub span: Span,
}
