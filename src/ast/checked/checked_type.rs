use crate::ast::{base::base_declaration::EnumDecl, Span};

use super::checked_declaration::{
    CheckedGenericParam, CheckedParam, CheckedStructDecl, CheckedTypeAliasDecl,
    SpecializedStructDecl,
};

#[derive(Clone, Debug)]
pub enum StructTypeKind {
    Declaration(CheckedStructDecl),
    Specialized(SpecializedStructDecl),
}

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
    Struct(StructTypeKind),
    Enum(EnumDecl),
    GenericParam(CheckedGenericParam),
    TypeAlias(CheckedTypeAliasDecl),
    FnType {
        params: Vec<CheckedParam>,
        return_type: Box<Type>,
        generic_params: Vec<CheckedGenericParam>,
    },
    // Infix types
    Union(Vec<Type>),
    // Suffix types
    Array {
        item_type: Box<Type>,
        size: usize,
    },
    Unknown,
}

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
            (
                TypeKind::Struct(StructTypeKind::Declaration(CheckedStructDecl {
                    identifier: this_identifier,
                    properties: this_properties,
                    generic_params: this_generic_params,
                    documentation: _,
                })),
                TypeKind::Struct(StructTypeKind::Declaration(CheckedStructDecl {
                    identifier: other_identifier,
                    properties: other_properties,
                    generic_params: other_generic_params,
                    documentation: _,
                })),
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
                TypeKind::GenericParam(CheckedGenericParam {
                    identifier: this_identifier,
                    constraint: this_constraint,
                }),
                TypeKind::GenericParam(CheckedGenericParam {
                    identifier: other_identifier,
                    constraint: other_constraint,
                }),
            ) => {
                this_identifier.name == other_identifier.name && this_constraint == other_constraint
            }
            (
                TypeKind::TypeAlias(CheckedTypeAliasDecl {
                    identifier: this_identifier,
                    generic_params: this_generic_params,
                    value: this_value,
                    documentation: _,
                }),
                TypeKind::TypeAlias(CheckedTypeAliasDecl {
                    identifier: other_identifier,
                    generic_params: other_generic_params,
                    value: other_value,
                    documentation: _,
                }),
            ) => {
                let same_name = this_identifier.name == other_identifier.name;

                if this_generic_params.len() != other_generic_params.len() {
                    return false;
                }

                let same_generic_params = this_generic_params
                    .iter()
                    .zip(other_generic_params.iter())
                    .all(|(this_param, other_param)| {
                        this_param.constraint == other_param.constraint
                    });

                let same_value = this_value == other_value;

                same_name && same_generic_params && same_value
            }
            (
                TypeKind::Enum(EnumDecl {
                    identifier: this_identifier,
                    variants: this_variants,
                    documentation: _,
                }),
                TypeKind::Enum(EnumDecl {
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
            (
                TypeKind::FnType {
                    params: this_params,
                    return_type: this_return_type,
                    generic_params: this_generic_params,
                },
                TypeKind::FnType {
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
            (TypeKind::Union(left_items), TypeKind::Union(right_items)) => {
                let same_len = left_items.len() == right_items.len();

                let same_items = left_items.iter().all(|item| right_items.contains(item))
                    && right_items.iter().all(|item| left_items.contains(item));

                same_len && same_items
            }
            (
                TypeKind::Array {
                    item_type: this_left,
                    size: this_size,
                },
                TypeKind::Array {
                    item_type: other_left,
                    size: other_size,
                },
            ) => this_left == other_left && this_size == other_size,
            (TypeKind::Unknown, TypeKind::Unknown) => true,
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
pub struct Type {
    pub kind: TypeKind,
    pub span: TypeSpan,
}

impl Type {
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

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}
