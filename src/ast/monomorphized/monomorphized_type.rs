use crate::ast::base::base_declaration::EnumDecl;

use super::monomorphized_declaration::{MonoParam, MonoStructDecl, MonoTypeAliasDecl};

#[derive(Clone, Debug)]
pub enum MonoType {
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
    Struct(MonoStructDecl),
    Enum(EnumDecl),
    TypeAlias(MonoTypeAliasDecl),
    FnType {
        params: Vec<MonoParam>,
        return_type: Box<MonoType>,
    },
    // Infix types
    Union(Vec<MonoType>),
    // Suffix types
    Array {
        item_type: Box<MonoType>,
        size: usize,
    },
    Unknown,
}

impl PartialEq for MonoType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MonoType::Void, MonoType::Void) => true,
            (MonoType::Null, MonoType::Null) => true,
            (MonoType::Bool, MonoType::Bool) => true,
            (MonoType::U8, MonoType::U8) => true,
            (MonoType::U16, MonoType::U16) => true,
            (MonoType::U32, MonoType::U32) => true,
            (MonoType::U64, MonoType::U64) => true,
            (MonoType::USize, MonoType::USize) => true,
            (MonoType::ISize, MonoType::ISize) => true,
            (MonoType::I8, MonoType::I8) => true,
            (MonoType::I16, MonoType::I16) => true,
            (MonoType::I32, MonoType::I32) => true,
            (MonoType::I64, MonoType::I64) => true,
            (MonoType::F32, MonoType::F32) => true,
            (MonoType::F64, MonoType::F64) => true,
            (MonoType::Char, MonoType::Char) => true,
            (
                MonoType::Struct(MonoStructDecl {
                    identifier: this_identifier,
                    properties: this_properties,
                }),
                MonoType::Struct(MonoStructDecl {
                    identifier: other_identifier,
                    properties: other_properties,
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

                same_name && same_props
            }
            (
                MonoType::TypeAlias(MonoTypeAliasDecl {
                    identifier: this_identifier,
                    value: this_value,
                }),
                MonoType::TypeAlias(MonoTypeAliasDecl {
                    identifier: other_identifier,
                    value: other_value,
                }),
            ) => {
                let same_name = this_identifier.name == other_identifier.name;

                let same_value = this_value == other_value;

                same_name && same_value
            }
            (
                MonoType::Enum(EnumDecl {
                    identifier: this_identifier,
                    variants: this_variants,
                    documentation: _,
                }),
                MonoType::Enum(EnumDecl {
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
                MonoType::FnType {
                    params: this_params,
                    return_type: this_return_type,
                },
                MonoType::FnType {
                    params: other_params,
                    return_type: other_return_type,
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

                let same_return_type = this_return_type == other_return_type;

                same_params && same_return_type
            }
            (MonoType::Union(left_items), MonoType::Union(right_items)) => {
                let same_len = left_items.len() == right_items.len();

                let same_items = left_items.iter().all(|item| right_items.contains(item))
                    && right_items.iter().all(|item| left_items.contains(item));

                same_len && same_items
            }
            (
                MonoType::Array {
                    item_type: this_left,
                    size: this_size,
                },
                MonoType::Array {
                    item_type: other_left,
                    size: other_size,
                },
            ) => this_left == other_left && this_size == other_size,
            (MonoType::Unknown, MonoType::Unknown) => true,
            _ => false,
        }
    }
}
