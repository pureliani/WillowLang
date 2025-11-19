use crate::hir::types::checked_type::Type;

pub struct Layout {
    pub size: usize,
    pub alignment: usize,
}

impl Layout {
    pub fn new(size: usize, alignment: usize) -> Self {
        Self { size, alignment }
    }
}

pub fn get_layout_of(ty: &Type) -> Layout {
    match ty {
        Type::Void => Layout::new(0, 1),
        Type::Bool | Type::U8 | Type::I8 => Layout::new(1, 1),
        Type::U16 | Type::I16 => Layout::new(2, 2),
        Type::U32 | Type::I32 | Type::F32 => Layout::new(4, 4),
        Type::U64 | Type::I64 | Type::F64 => Layout::new(8, 8),
        // Pointers and USize depend on target arch, assuming 64-bit for now
        Type::Pointer(_) | Type::Fn(_) | Type::USize | Type::ISize => Layout::new(8, 8),

        Type::Buffer { size, alignment } => Layout::new(*size, *alignment),

        Type::Unknown => Layout::new(0, 1), // Should probably panic or handle error

        Type::Struct(s) => {
            // For UserDefined and ClosureEnv, fields are packed (reordered)
            // For others (List, Tag, Union), fields are fixed
            // `get_layout_of` calculates the layout of the *resulting* struct
            // _after_ packing has already happened

            let mut current_offset = 0;
            let mut max_alignment = 1;

            for field in s.fields() {
                let field_layout = get_layout_of(&field.ty);

                max_alignment = std::cmp::max(max_alignment, field_layout.alignment);

                // Add padding to align the current field
                let padding = (field_layout.alignment
                    - (current_offset % field_layout.alignment))
                    % field_layout.alignment;
                current_offset += padding;

                // Add field size
                current_offset += field_layout.size;
            }

            // The total size of a struct must be a multiple of its alignment
            let padding_end =
                (max_alignment - (current_offset % max_alignment)) % max_alignment;
            let total_size = current_offset + padding_end;

            Layout::new(total_size, max_alignment)
        }
    }
}

pub fn get_alignment_of(ty: &Type) -> usize {
    get_layout_of(ty).alignment
}
