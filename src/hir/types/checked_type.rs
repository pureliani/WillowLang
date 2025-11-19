use crate::{
    ast::{IdentifierNode, Span},
    compile::interner::StringId,
    hir::{
        types::checked_declaration::{CheckedFnType, CheckedParam},
        utils::{layout::get_layout_of, pack_struct::pack_struct},
        ProgramBuilder,
    },
};
use std::hash::Hash;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StructKind {
    /// Packed by "pack_struct" helper
    UserDefined,
    /// { fn_ptr, env_ptr }
    Closure,
    /// Two closures capturing the same variables (names + types + order)
    /// will have the same Environment Type.
    ClosureEnv,
    /// { discriminant, value }
    Tag,
    /// { discriminant, payload_union }
    Union,
    /// { capacity, len, ptr }
    List,
    /// { capacity, len, ptr }
    String,
    /// { len, ptr }
    ConstString,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CheckedStruct {
    kind: StructKind,
    fields: Vec<CheckedParam>,
}

impl CheckedStruct {
    pub fn kind(&self) -> &StructKind {
        &self.kind
    }

    pub fn fields(&self) -> &[CheckedParam] {
        &self.fields
    }

    /// Looks up a field by name.
    /// Returns `Some((index, type))` if found, or `None`.
    ///
    /// This is used by the FunctionBuilder to emit `GetFieldPtr` instructions.
    pub fn get_field(&self, name: StringId) -> Option<(usize, &Type)> {
        self.fields.iter().enumerate().find_map(|(index, param)| {
            if param.identifier.name == name {
                Some((index, &param.ty))
            } else {
                None
            }
        })
    }

    /// Helper to create a param with a default span (for internal types)
    fn internal_param(name: StringId, ty: Type) -> CheckedParam {
        CheckedParam {
            identifier: IdentifierNode {
                name,
                span: Span::default(),
            },
            ty,
        }
    }

    pub fn user_defined(
        program_builder: &ProgramBuilder,
        fields: &mut [CheckedParam],
    ) -> Self {
        pack_struct(program_builder, fields);
        Self {
            kind: StructKind::UserDefined,
            fields: fields.into(),
        }
    }

    /// Creates the "Fat Pointer" wrapper: { fn_ptr: *void, env_ptr: *void }
    pub fn closure(program_builder: &ProgramBuilder) -> Self {
        let void_ptr = Type::Pointer(Box::new(Type::Void));

        // Fixed order
        let fields = vec![
            Self::internal_param(
                program_builder.common_identifiers.fn_ptr,
                void_ptr.clone(),
            ),
            Self::internal_param(program_builder.common_identifiers.env_ptr, void_ptr),
        ];

        Self {
            kind: StructKind::Closure,
            fields,
        }
    }

    /// Creates the captured environment struct.
    /// Packed to minimize size.
    pub fn closure_env(
        program_builder: &ProgramBuilder,
        fields: &mut [CheckedParam],
    ) -> Self {
        pack_struct(program_builder, fields);
        Self {
            kind: StructKind::ClosureEnv,
            fields: fields.into(),
        }
    }

    /// Creates a Tag struct: { id: u16, value: T }
    /// IMPORTANT: We do NOT pack this. The `id` must be at offset 0
    /// so it aligns with the Union's `id`
    pub fn tag(program_builder: &ProgramBuilder, value_type: Option<Type>) -> Self {
        let mut fields = vec![Self::internal_param(
            program_builder.common_identifiers.id,
            Type::U16,
        )];

        if let Some(ty) = value_type {
            fields.push(Self::internal_param(
                program_builder.common_identifiers.value,
                ty,
            ));
        }

        Self {
            kind: StructKind::Tag,
            fields,
        }
    }

    /// Creates a Union struct: { id: u16, payload: Buffer }
    /// The payload is large enough to hold the largest variant.
    pub fn union(program_builder: &ProgramBuilder, variants: &[Type]) -> Self {
        let mut max_size = 0;
        let mut max_align = 1;

        for variant_type in variants {
            let layout = get_layout_of(variant_type);
            if layout.size > max_size {
                max_size = layout.size;
            }
            if layout.alignment > max_align {
                max_align = layout.alignment;
            }
        }

        let payload_type = Type::Buffer {
            size: max_size,
            alignment: max_align,
        };

        let fields = vec![
            Self::internal_param(program_builder.common_identifiers.id, Type::U16),
            Self::internal_param(
                program_builder.common_identifiers.payload,
                payload_type,
            ),
        ];

        Self {
            kind: StructKind::Union,
            fields,
        }
    }

    /// Creates a dynamic List: { capacity: usize, len: usize, ptr: *T }
    pub fn list(program_builder: &ProgramBuilder, element_type: Type) -> Self {
        // Fixed order: capacity, len, ptr
        let fields = vec![
            Self::internal_param(
                program_builder.common_identifiers.capacity,
                Type::USize,
            ),
            Self::internal_param(program_builder.common_identifiers.len, Type::USize),
            Self::internal_param(
                program_builder.common_identifiers.ptr,
                Type::Pointer(Box::new(element_type)),
            ),
        ];

        Self {
            kind: StructKind::List,
            fields,
        }
    }

    /// Creates a dynamic String: { capacity: usize, len: usize, ptr: *u8 }
    pub fn string(program_builder: &ProgramBuilder) -> Self {
        // Fixed order: capacity, len, ptr
        let fields = vec![
            Self::internal_param(
                program_builder.common_identifiers.capacity,
                Type::USize,
            ),
            Self::internal_param(program_builder.common_identifiers.len, Type::USize),
            Self::internal_param(
                program_builder.common_identifiers.ptr,
                Type::Pointer(Box::new(Type::U8)),
            ),
        ];

        Self {
            kind: StructKind::String,
            fields,
        }
    }

    /// Creates a String View (ConstString): { len: usize, ptr: *u8 }
    pub fn const_string(program_builder: &ProgramBuilder) -> Self {
        // Fixed order
        let fields = vec![
            Self::internal_param(program_builder.common_identifiers.len, Type::USize),
            Self::internal_param(
                program_builder.common_identifiers.ptr,
                Type::Pointer(Box::new(Type::U8)),
            ),
        ];

        Self {
            kind: StructKind::ConstString,
            fields,
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
    /// (User structs, Lists, Strings, Closures, Unions, etc..)
    Struct(CheckedStruct),

    /// Represents a function pointer signature
    Fn(CheckedFnType),

    /// Represents a raw block of memory with a specific size and alignment.
    /// Used for Union payloads.
    Buffer {
        size: usize,
        alignment: usize,
    },

    /// Used for error recovery.
    Unknown,
}
