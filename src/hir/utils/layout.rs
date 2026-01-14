use std::panic;

use crate::hir::types::checked_declaration::CheckedParam;
use crate::hir::types::checked_type::{StructKind, Type};
use crate::hir::ProgramBuilder;

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
const PTR_SIZE: usize = size_of::<usize>();
const PTR_ALIGN: usize = size_of::<usize>();
const USIZE_SIZE: usize = size_of::<usize>();
const USIZE_ALIGN: usize = size_of::<usize>();

/// IMPORTANT: Make sure user-defined and closure-environment structs are packed first before calling this function
pub fn get_layout_of(ty: &Type, ctx: &ProgramBuilder) -> Layout {
    match ty {
        Type::Void => Layout::new(0, 1),
        Type::Bool | Type::U8 | Type::I8 => Layout::new(1, 1),
        Type::U16 | Type::I16 => Layout::new(2, 2),
        Type::U32 | Type::I32 | Type::F32 => Layout::new(4, 4),
        Type::U64 | Type::I64 | Type::F64 => Layout::new(8, 8),

        Type::Pointer { .. } | Type::Fn(_) | Type::USize | Type::ISize => {
            Layout::new(USIZE_SIZE, USIZE_ALIGN)
        }

        Type::Buffer { size, alignment } => Layout::new(*size, *alignment),

        Type::Unknown => Layout::new(0, 1),

        Type::Struct(s) => {
            let fields = s.fields(ctx);
            let types: Vec<&Type> = fields.iter().map(|(_, ty)| ty).collect();

            calculate_fields_layout(&types, ctx)
        }
    }
}

pub fn get_alignment_of(ty: &Type, ctx: &ProgramBuilder) -> usize {
    get_layout_of(ty, ctx).alignment
}

/// Helper to calculate layout of fields placed sequentially in memory
fn calculate_fields_layout(field_types: &[&Type], ctx: &ProgramBuilder) -> Layout {
    let mut current_offset = 0;
    let mut max_alignment = 1;

    for ty in field_types {
        let field_layout = get_layout_of(ty, ctx);

        max_alignment = std::cmp::max(max_alignment, field_layout.alignment);

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

pub fn pack_struct(
    program_builder: &ProgramBuilder,
    struct_kind: StructKind,
) -> StructKind {
    match struct_kind {
        StructKind::UserDefined(mut fields) => {
            sort_fields(program_builder, &mut fields);
            StructKind::UserDefined(fields)
        }
        _ => {
            panic!("INTERNAL COMPILER ERROR: Cannot pack struct that is neither user defined nor closure environment!");
        }
    }
}

fn sort_fields(program_builder: &ProgramBuilder, fields: &mut [CheckedParam]) {
    fields.sort_by(|field_a, field_b| {
        let align_a = get_alignment_of(&field_a.ty, program_builder);
        let align_b = get_alignment_of(&field_b.ty, program_builder);

        // Sort by Alignment (Descending) -> Name (Ascending)
        align_b.cmp(&align_a).then_with(|| {
            let name_a = program_builder
                .string_interner
                .resolve(field_a.identifier.name);
            let name_b = program_builder
                .string_interner
                .resolve(field_b.identifier.name);

            name_a.cmp(&name_b)
        })
    });
}
