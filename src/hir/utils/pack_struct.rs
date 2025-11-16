use crate::hir::{
    types::{checked_declaration::CheckedParam, checked_type::TypeKind},
    HIRContext,
};

pub fn get_alignment_of(type_kind: &TypeKind) -> usize {
    use std::mem::align_of;

    match type_kind {
        TypeKind::U64 | TypeKind::I64 | TypeKind::F64 => 8,
        TypeKind::U32 | TypeKind::I32 | TypeKind::F32 => 4,
        TypeKind::U16 | TypeKind::I16 => 2,
        TypeKind::U8 | TypeKind::I8 | TypeKind::Bool => 1,

        TypeKind::Pointer(_)
        | TypeKind::USize
        | TypeKind::ISize
        | TypeKind::FnType(_) // A function pointer or closure object pointer
        | TypeKind::Struct(_) // A pointer to a struct on the heap
        | TypeKind::List(_)   // A pointer to a list object on the heap
        | TypeKind::String => align_of::<usize>(),

        TypeKind::Tag(checked_tag_type) => {
            let discriminant_align = get_alignment_of(&TypeKind::U16); // Tag header is u16

            checked_tag_type
                .value_type
                .as_ref()
                .map(|value| get_alignment_of(&value.kind))
                .unwrap_or(1)
                .max(discriminant_align)
        }
        TypeKind::Union(checked_tag_types) => {
            let discriminant_align = get_alignment_of(&TypeKind::U16); // Tag header is u16

            // Find the maximum alignment required by any of the possible value
            let max_variant_align = checked_tag_types
                .iter()
                .map(|tag| {
                    tag.value_type
                        .as_ref()
                        .map(|value| get_alignment_of(&value.kind))
                        .unwrap_or(1)
                })
                .max()
                .unwrap_or(1);

            max_variant_align.max(discriminant_align)
        }

        TypeKind::Void => 1, // Void has size 0 but we can give it a minimum alignment of 1
        TypeKind::Unknown => 1, // Avoid panicking on errors.
    }
}

pub fn pack_struct(ctx: &HIRContext, fields: &mut [CheckedParam]) {
    fields.sort_by(|field_a, field_b| {
        let align_a = get_alignment_of(&field_a.ty.kind);
        let align_b = get_alignment_of(&field_b.ty.kind);

        align_b.cmp(&align_a).then_with(|| {
            let name_a = ctx
                .program_builder
                .string_interner
                .resolve(field_a.identifier.name);
            let name_b = ctx
                .program_builder
                .string_interner
                .resolve(field_b.identifier.name);

            name_a.cmp(&name_b)
        })
    });
}
