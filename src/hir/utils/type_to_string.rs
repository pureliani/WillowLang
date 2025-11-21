use std::collections::HashSet;

use crate::{
    compile::interner::StringId,
    hir::{
        types::{
            checked_declaration::{CheckedParam, FnType, TagType},
            checked_type::{StructKind, Type},
        },
        ProgramBuilder,
    },
};

fn identifier_to_string(id: StringId, program_builder: &ProgramBuilder) -> String {
    // SharedInterner::resolve returns T (String), so we get an owned String here.
    program_builder.string_interner.resolve(id)
}

fn tag_type_to_string(
    tag: &TagType,
    program_builder: &ProgramBuilder,
    visited_set: &mut HashSet<Type>,
) -> String {
    // 1. Resolve TagId -> StringId
    let name_string_id = program_builder.tag_interner.resolve(tag.id);
    // 2. Resolve StringId -> String
    let name = program_builder.string_interner.resolve(name_string_id);

    // 3. Format value if present
    let value_str = tag
        .value_type
        .as_ref()
        .map(|v| {
            format!(
                "({})",
                type_to_string_recursive(v, program_builder, visited_set)
            )
        })
        .unwrap_or_default();

    format!("#{0}{1}", name, value_str)
}

pub fn type_to_string(ty: &Type, program_builder: &ProgramBuilder) -> String {
    let mut visited_set = HashSet::new();
    type_to_string_recursive(ty, program_builder, &mut visited_set)
}

pub fn type_to_string_recursive(
    ty: &Type,
    program_builder: &ProgramBuilder,
    visited_set: &mut HashSet<Type>,
) -> String {
    if !visited_set.insert(ty.clone()) {
        return "...".to_string();
    }

    let result = match ty {
        Type::Void => String::from("void"),
        Type::Bool => String::from("bool"),
        Type::U8 => String::from("u8"),
        Type::U16 => String::from("u16"),
        Type::U32 => String::from("u32"),
        Type::U64 => String::from("u64"),
        Type::USize => String::from("usize"),
        Type::ISize => String::from("isize"),
        Type::I8 => String::from("i8"),
        Type::I16 => String::from("i16"),
        Type::I32 => String::from("i32"),
        Type::I64 => String::from("i64"),
        Type::F32 => String::from("f32"),
        Type::F64 => String::from("f64"),
        Type::Unknown => String::from("unknown"),

        Type::Struct(s) => struct_to_string(s, program_builder, visited_set),

        Type::Fn(fn_type) => {
            fn_signature_to_string(fn_type, program_builder, visited_set)
        }

        Type::Pointer(ty) => format!(
            "ptr<{}>",
            type_to_string_recursive(ty, program_builder, visited_set)
        ),
        Type::Buffer { size, alignment } => {
            format!("Buffer(size={}, align={})", size, alignment)
        }
    };

    visited_set.remove(ty);

    result
}

fn fn_signature_to_string(
    fn_type: &FnType,
    program_builder: &ProgramBuilder,
    visited_set: &mut HashSet<Type>,
) -> String {
    let params_str = fn_type
        .params
        .iter()
        .map(|p| {
            format!(
                "{}: {}",
                identifier_to_string(p.identifier.name, program_builder),
                type_to_string_recursive(&p.ty, program_builder, visited_set)
            )
        })
        .collect::<Vec<String>>()
        .join(", ");

    let return_type_str =
        type_to_string_recursive(&fn_type.return_type, program_builder, visited_set);

    format!("fn({}): {}", params_str, return_type_str)
}

fn struct_to_string(
    s: &StructKind,
    program_builder: &ProgramBuilder,
    visited_set: &mut HashSet<Type>,
) -> String {
    match s {
        StructKind::UserDefined(fields) => {
            let fields_str = fields
                .iter()
                .map(|f| {
                    format!(
                        "{}: {}",
                        identifier_to_string(f.identifier.name, program_builder),
                        type_to_string_recursive(&f.ty, program_builder, visited_set)
                    )
                })
                .collect::<Vec<String>>()
                .join(", ");
            format!("{{ {} }}", fields_str)
        }
        StructKind::Closure(fn_type) => {
            format!(
                "Closure<{}>",
                fn_signature_to_string(fn_type, program_builder, visited_set)
            )
        }
        StructKind::ClosureEnv(fields) => {
            let fields_str = fields
                .iter()
                .map(|f| {
                    format!(
                        "{}: {}",
                        identifier_to_string(f.identifier.name, program_builder),
                        type_to_string_recursive(&f.ty, program_builder, visited_set)
                    )
                })
                .collect::<Vec<String>>()
                .join(", ");
            format!("ClosureEnv {{ {} }}", fields_str)
        }
        StructKind::Tag(tag_type) => {
            tag_type_to_string(tag_type, program_builder, visited_set)
        }
        StructKind::Union { variants } => {
            if variants.is_empty() {
                return String::from("<empty union>");
            }
            let variants_str = variants
                .iter()
                .map(|tag| tag_type_to_string(tag, program_builder, visited_set))
                .collect::<Vec<String>>()
                .join(" | ");

            variants_str
        }
        StructKind::List(item_type) => {
            let elem_type_str =
                type_to_string_recursive(item_type, program_builder, visited_set);

            format!("{}[]", elem_type_str)
        }
        StructKind::String | StructKind::ConstString => String::from("string"),
    }
}
