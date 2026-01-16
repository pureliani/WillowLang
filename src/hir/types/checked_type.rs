use crate::{
    compile::interner::StringId,
    hir::{
        types::checked_declaration::{CheckedParam, FnType, TagType},
        utils::layout::get_layout_of,
        ProgramBuilder,
    },
};
use std::hash::Hash;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StructKind {
    UserDefined(Vec<CheckedParam>), // packed

    /// { id: u16, value: T }
    Tag(TagType),

    /// { id: u16, value: Buffer }
    Union {
        variants: Vec<TagType>,
    },

    /// { capacity: usize, len: usize, ptr: ptr<T> }
    List(Box<Type>),

    /// { is_heap_allocated: bool, len: usize, ptr: ptr<u8> }
    String,
}

impl StructKind {
    pub fn fields(&self, ctx: &ProgramBuilder) -> Vec<(StringId, Type)> {
        match self {
            StructKind::UserDefined(params) => params
                .iter()
                .map(|p| (p.identifier.name, p.ty.clone()))
                .collect(),

            StructKind::List(elem_ty) => vec![
                (ctx.common_identifiers.capacity, Type::USize),
                (ctx.common_identifiers.len, Type::USize),
                (ctx.common_identifiers.ptr, Type::Pointer(elem_ty.clone())),
            ],

            StructKind::String => vec![
                (ctx.common_identifiers.is_heap_allocated, Type::Bool),
                (ctx.common_identifiers.len, Type::USize),
                (
                    ctx.common_identifiers.ptr,
                    Type::Pointer(Box::new(Type::U8)),
                ),
            ],

            StructKind::Tag(tag) => {
                let mut fields = vec![(ctx.common_identifiers.id, Type::U16)];
                if let Some(val) = &tag.value_type {
                    fields.push((ctx.common_identifiers.value, *val.clone()));
                }
                fields
            }

            StructKind::Union { variants } => {
                let mut max_size = 0;
                let mut max_align = 1;

                for tag_type in variants {
                    if let Some(val_ty) = &tag_type.value_type {
                        let layout = get_layout_of(val_ty, ctx);
                        max_size = std::cmp::max(max_size, layout.size);
                        max_align = std::cmp::max(max_align, layout.alignment);
                    }
                }

                vec![
                    (ctx.common_identifiers.id, Type::U16),
                    (
                        ctx.common_identifiers.value,
                        Type::Buffer {
                            size: max_size,
                            alignment: max_align,
                        },
                    ),
                ]
            }
        }
    }

    /// Maps a Field Name -> (Index, Type).
    pub fn get_field(
        &self,
        ctx: &ProgramBuilder,
        name: StringId,
    ) -> Option<(usize, Type)> {
        self.fields(ctx)
            .into_iter()
            .enumerate()
            .find(|(_, (field_name, _))| *field_name == name)
            .map(|(index, (_, ty))| (index, ty))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Type {
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

    /// Represents a pointer to another type
    Pointer(Box<Type>),

    /// Represents any block of memory with named fields
    Struct(StructKind),

    /// Represents a function pointer signature
    Fn(FnType),

    /// Represents a raw block of memory with a specific size and alignment
    Buffer {
        size: usize,
        alignment: usize,
    },

    /// Used for error recovery
    Unknown,
}
