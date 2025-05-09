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
            // TODO: handle non generic struct
            (
                CheckedTypeKind::GenericStructDecl(GenericStructDecl {
                    identifier: this_identifier,
                    properties: this_properties,
                    generic_params: this_generic_params,
                    documentation: _,
                }),
                CheckedTypeKind::GenericStructDecl(GenericStructDecl {
                    identifier: other_identifier,
                    properties: other_properties,
                    generic_params: other_generic_params,
                    documentation: _,
                }),
            ) => {
                let same_name = this_identifier.name == other_identifier.name;

                if this_properties.len() != other_properties.len() {
                    return false;
                }

                let same_props = this_properties.iter().zip(other_properties.iter()).all(
                    |(this_param, other_param)| {
                        this_param.identifier.name == other_param.identifier.name
                            && this_param.constraint == other_param.constraint
                    },
                );

                if this_generic_params.len() != other_generic_params.len() {
                    return false;
                }

                let same_generic_params = this_generic_params
                    .iter()
                    .zip(other_generic_params.iter())
                    .all(|(this_param, other_param)| {
                        this_param.constraint == other_param.constraint
                    });

                same_name && same_props && same_generic_params
            }
            (
                CheckedTypeKind::GenericParam(CheckedGenericParam {
                    identifier: this_identifier,
                    constraint: this_constraint,
                }),
                CheckedTypeKind::GenericParam(CheckedGenericParam {
                    identifier: other_identifier,
                    constraint: other_constraint,
                }),
            ) => {
                this_identifier.name == other_identifier.name && this_constraint == other_constraint
            }
            (
                CheckedTypeKind::Enum(EnumDecl {
                    identifier: this_identifier,
                    variants: this_variants,
                    documentation: _,
                }),
                CheckedTypeKind::Enum(EnumDecl {
                    identifier: other_identifier,
                    variants: other_variants,
                    documentation: _,
                }),
            ) => {
                this_identifier.name == other_identifier.name
                    && this_variants.iter().zip(other_variants.iter()).all(
                        |(this_variant, other_variant)| this_variant.name == other_variant.name,
                    )
            }
            // TODO: handle non generic fn
            (
                CheckedTypeKind::GenericFnType {
                    params: this_params,
                    return_type: this_return_type,
                    generic_params: this_generic_params,
                },
                CheckedTypeKind::GenericFnType {
                    params: other_params,
                    return_type: other_return_type,
                    generic_params: other_generic_params,
                },
            ) => {
                if this_params.len() != other_params.len() {
                    return false;
                }

                let same_params =
                    this_params
                        .iter()
                        .zip(other_params.iter())
                        .all(|(this_param, other_param)| {
                            this_param.identifier.name == other_param.identifier.name
                                && this_param.constraint == other_param.constraint
                        });

                if this_generic_params.len() != other_generic_params.len() {
                    return false;
                }

                let same_generic_params = this_generic_params
                    .iter()
                    .zip(other_generic_params.iter())
                    .all(|(this_param, other_param)| {
                        this_param.constraint == other_param.constraint
                    });

                let same_return_type = this_return_type == other_return_type;

                same_params && same_generic_params && same_return_type
            }
            (CheckedTypeKind::Union(left_items), CheckedTypeKind::Union(right_items)) => {
                let same_len = left_items.len() == right_items.len();

                let same_items = left_items.iter().all(|item| right_items.contains(item))
                    && right_items.iter().all(|item| left_items.contains(item));

                same_len && same_items
            }
            (
                CheckedTypeKind::Array {
                    item_type: this_left,
                    size: this_size,
                },
                CheckedTypeKind::Array {
                    item_type: other_left,
                    size: other_size,
                },
            ) => this_left == other_left && this_size == other_size,
            (CheckedTypeKind::Unknown, CheckedTypeKind::Unknown) => true,
            _ => false,
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

impl PartialEq for CheckedType {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}
