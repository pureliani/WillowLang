use crate::{
    ast::{IdentifierNode, Span},
    parse::ParsingError,
    tokenizer::NumberKind,
};

use super::base_declaration::{GenericParam, Param};

#[derive(Clone, Debug, PartialEq)]
pub enum TypeAnnotationKind {
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
    Identifier(IdentifierNode),
    GenericFnType {
        params: Vec<Param>,
        return_type: Box<TypeAnnotation>,
        generic_params: Vec<GenericParam>,
    },
    FnType {
        params: Vec<Param>,
        return_type: Box<TypeAnnotation>,
    },
    // Infix types
    Union(Vec<TypeAnnotation>),
    // Suffix types
    Array {
        left: Box<TypeAnnotation>,
        size: NumberKind,
    },
    GenericApply {
        left: Box<TypeAnnotation>,
        args: Vec<TypeAnnotation>,
    },
    Error(ParsingError),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeAnnotation {
    pub kind: TypeAnnotationKind,
    pub span: Span,
}
