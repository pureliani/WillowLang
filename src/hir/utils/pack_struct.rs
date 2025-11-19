use crate::hir::{
    types::checked_declaration::CheckedParam, utils::layout::get_alignment_of,
    ProgramBuilder,
};

pub fn pack_struct(program_builder: &ProgramBuilder, fields: &mut [CheckedParam]) {
    fields.sort_by(|field_a, field_b| {
        let align_a = get_alignment_of(&field_a.ty);
        let align_b = get_alignment_of(&field_b.ty);

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
