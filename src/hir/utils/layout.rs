use crate::hir::types::checked_type::{StructKind, Type};
use std::cmp;

pub struct Layout {
    pub size: usize,
    pub alignment: usize,
}

impl Layout {
    pub fn new(size: usize, alignment: usize) -> Self {
        Self { size, alignment }
    }
}

// Constants for the target architecture
const PTR_SIZE: usize = 8;
const PTR_ALIGN: usize = 8;
const USIZE_SIZE: usize = 8;
const USIZE_ALIGN: usize = 8;

/// IMPORTANT: Make sure user-defined and closure-environment structs are packed first before calling this function
pub fn get_layout_of(ty: &Type) -> Layout {
    match ty {
        Type::Void => Layout::new(0, 1),
        Type::Bool | Type::U8 | Type::I8 => Layout::new(1, 1),
        Type::U16 | Type::I16 => Layout::new(2, 2),
        Type::U32 | Type::I32 | Type::F32 => Layout::new(4, 4),
        Type::U64 | Type::I64 | Type::F64 => Layout::new(8, 8),

        Type::Pointer(_) | Type::Fn(_) | Type::USize | Type::ISize => {
            Layout::new(USIZE_SIZE, USIZE_ALIGN)
        }

        Type::Buffer { size, alignment } => Layout::new(*size, *alignment),

        Type::Unknown => Layout::new(0, 1),

        Type::Struct(s) => get_struct_layout(s),
    }
}

pub fn get_alignment_of(ty: &Type) -> usize {
    get_layout_of(ty).alignment
}

fn get_struct_layout(s: &StructKind) -> Layout {
    match s {
        // Must be packed before
        StructKind::UserDefined(fields) | StructKind::ClosureEnv(fields) => {
            let types: Vec<&Type> = fields.iter().map(|f| &f.ty).collect();
            calculate_fields_layout(&types)
        }

        // { fn_ptr, env_ptr }
        StructKind::Closure(_) => Layout::new(PTR_SIZE * 2, PTR_ALIGN),

        // { capacity: usize, len: usize, ptr: ptr<T> }
        StructKind::List(_) => {
            let size = USIZE_SIZE + USIZE_SIZE + PTR_SIZE;
            Layout::new(size, USIZE_ALIGN)
        }

        // { capacity: usize, len: usize, ptr: ptr<u8> }
        StructKind::String => {
            let size = USIZE_SIZE + USIZE_SIZE + PTR_SIZE;
            Layout::new(size, USIZE_ALIGN)
        }

        // { len: usize, ptr: ptr<u8> }
        StructKind::ConstString => {
            let size = USIZE_SIZE + PTR_SIZE;
            Layout::new(size, USIZE_ALIGN)
        }

        // { id: u16, value: T }
        StructKind::Tag(tag_type) => {
            let mut types = vec![&Type::U16];
            if let Some(val_ty) = &tag_type.value_type {
                types.push(val_ty);
            }

            calculate_fields_layout(&types)
        }

        // { id: u16, payload: Buffer }
        StructKind::Union { variants } => {
            let mut max_size = 0;
            let mut max_align = 1;

            for tag_type in variants {
                let tag_struct = Type::Struct(StructKind::Tag(tag_type.clone()));
                let layout = get_layout_of(&tag_struct);

                max_size = cmp::max(max_size, layout.size);
                max_align = cmp::max(max_align, layout.alignment);
            }

            Layout::new(max_size, max_align)
        }
    }
}

/// Helper to calculate layout of fields placed sequentially in memory
fn calculate_fields_layout(field_types: &[&Type]) -> Layout {
    let mut current_offset = 0;
    let mut max_alignment = 1;

    for ty in field_types {
        let field_layout = get_layout_of(ty);

        max_alignment = cmp::max(max_alignment, field_layout.alignment);

        let padding = (field_layout.alignment
            - (current_offset % field_layout.alignment))
            % field_layout.alignment;

        current_offset += padding;
        current_offset += field_layout.size;
    }

    let padding_end = (max_alignment - (current_offset % max_alignment)) % max_alignment;
    let total_size = current_offset + padding_end;

    Layout::new(total_size, max_alignment)
}
