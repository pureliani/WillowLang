use crate::{
    compile::interner::StringId,
    hir::{
        types::checked_declaration::{CheckedParam, FnType, TagType},
        utils::layout::get_union_layout,
        ProgramBuilder,
    },
};
use std::hash::Hash;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StructKind {
    UserDefined(Vec<CheckedParam>), // packed

    /// { fn_ptr, env_ptr }
    ClosureObject(FnType),

    /// The captured environment.
    ClosureEnv(Vec<CheckedParam>), // packed

    /// { id: u16, value: T }
    Tag(TagType),

    /// { id: u16, payload: Buffer }
    Union {
        variants: Vec<TagType>,
    },

    /// { capacity: usize, len: usize, ptr: ptr<T> }
    List(Box<Type>),

    /// { capacity: usize, len: usize, ptr: ptr<u8> }
    String,

    /// { len: usize, ptr: ptr<u8> }
    ConstString,
}

impl StructKind {
    /// The "Magic" Lookup Method.
    /// Maps a Field Name -> (Index, Type).
    pub fn get_field(
        &self,
        ctx: &ProgramBuilder,
        name: StringId,
    ) -> Option<(usize, Type)> {
        match self {
            StructKind::UserDefined(fields) | StructKind::ClosureEnv(fields) => {
                fields.iter().enumerate().find_map(|(i, param)| {
                    if param.identifier.name == name {
                        Some((i, param.ty.clone()))
                    } else {
                        None
                    }
                })
            }

            StructKind::ClosureObject(_) => {
                let void_ptr = Type::Pointer(Box::new(Type::Void));
                if name == ctx.common_identifiers.fn_ptr {
                    Some((0, void_ptr.clone()))
                } else if name == ctx.common_identifiers.env_ptr {
                    Some((1, void_ptr))
                } else {
                    None
                }
            }

            // FIX: Destructure the tuple variant correctly
            StructKind::Tag(tag_type) => {
                if name == ctx.common_identifiers.id {
                    Some((0, Type::U16))
                } else if name == ctx.common_identifiers.value {
                    // Index 1 is the value (if it exists)
                    tag_type.value_type.as_ref().map(|t| (1, *t.clone()))
                } else {
                    None
                }
            }

            StructKind::Union { variants } => {
                if name == ctx.common_identifiers.id {
                    Some((0, Type::U16))
                } else if name == ctx.common_identifiers.payload {
                    let layout = get_union_layout(variants);

                    Some((
                        1,
                        Type::Buffer {
                            size: layout.size,
                            alignment: layout.alignment,
                        },
                    ))
                } else {
                    None
                }
            }

            StructKind::List(elem_ty) => {
                if name == ctx.common_identifiers.capacity {
                    Some((0, Type::USize))
                } else if name == ctx.common_identifiers.len {
                    Some((1, Type::USize))
                } else if name == ctx.common_identifiers.ptr {
                    Some((2, Type::Pointer(elem_ty.clone())))
                } else {
                    None
                }
            }

            StructKind::String => {
                if name == ctx.common_identifiers.capacity {
                    Some((0, Type::USize))
                } else if name == ctx.common_identifiers.len {
                    Some((1, Type::USize))
                } else if name == ctx.common_identifiers.ptr {
                    Some((2, Type::Pointer(Box::new(Type::U8))))
                } else {
                    None
                }
            }

            StructKind::ConstString => {
                if name == ctx.common_identifiers.len {
                    Some((0, Type::USize))
                } else if name == ctx.common_identifiers.ptr {
                    Some((1, Type::Pointer(Box::new(Type::U8))))
                } else {
                    None
                }
            }
        }
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
