use crate::ast::{IdentifierNode, Span};

use super::decl::Param;

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
    String,
    Identifier(IdentifierNode),
    List {
        item_type: Box<TypeAnnotation>,
    },
    FnType {
        params: Vec<Param>,
        return_type: Box<TypeAnnotation>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeAnnotation {
    pub kind: TypeAnnotationKind,
    pub span: Span,
}
