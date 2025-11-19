use std::collections::HashSet;

use crate::{
    compile::interner::StringId,
    hir::{
        types::{
            checked_declaration::CheckedFnType,
            checked_type::{CheckedStruct, StructKind, Type},
        },
        ProgramBuilder,
    },
};

fn identifier_to_string(id: StringId, program_builder: &ProgramBuilder) -> String {
    let identifier_name = program_builder.string_interner.resolve(id);
    identifier_name.to_owned()
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
    // If we are already visiting this type in the current path
    // return a placeholder to avoid infinite recursion.
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

        Type::Fn(CheckedFnType {
            params,
            return_type,
        }) => {
            let params_str = params
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
                type_to_string_recursive(&return_type, program_builder, visited_set);
            format!("fn ({}): {}", params_str, return_type_str)
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

fn struct_to_string(
    s: &CheckedStruct,
    program_builder: &ProgramBuilder,
    visited_set: &mut HashSet<Type>,
) -> String {
    match s.kind() {
        StructKind::UserDefined => {
            let fields = s
                .fields()
                .iter()
                .map(|f| {
                    format!(
                        "{}: {}",
                        identifier_to_string(f.identifier.name, program_builder,),
                        type_to_string_recursive(&f.ty, program_builder, visited_set)
                    )
                })
                .collect::<Vec<String>>()
                .join(", ");
            format!("{{ {} }}", fields)
        }
        StructKind::Closure => String::from("Closure"),
        StructKind::ClosureEnv => String::from("ClosureEnv"),
        StructKind::Tag => {
            // A Tag struct is { id: u16, value: T }.
            // We used the variant name as the field name for 'value'.
            // If there is a second field, we use its name as the tag name.
            if let Some(value_field) = s.fields().get(1) {
                let variant_name =
                    identifier_to_string(value_field.identifier.name, program_builder);
                let value_type = type_to_string_recursive(
                    &value_field.ty,
                    program_builder,
                    visited_set,
                );
                format!("#{}({})", variant_name, value_type)
            } else {
                // Unit tag (only has id field).
                // Note: We lost the name of the unit tag in the Type system!
                // We can only print a generic placeholder or the ID.
                String::from("#Tag")
            }
        }
        StructKind::Union => {
            // A Union struct is { id: u16, payload: Buffer }.
            // We lost the list of variants in the Type system.
            String::from("Union")
        }
        StructKind::List => {
            // List is { cap, len, ptr: Pointer<T> }.
            // We try to find the "ptr" field to determine T.
            let elem_type_str = s
                .fields()
                .iter()
                .find(|f| {
                    identifier_to_string(f.identifier.name, program_builder) == "ptr"
                })
                .map(|f| {
                    if let Type::Pointer(inner) = &f.ty {
                        type_to_string_recursive(inner, program_builder, visited_set)
                    } else {
                        String::from("?")
                    }
                })
                .unwrap_or_else(|| String::from("?"));

            format!("{}[]", elem_type_str)
        }
        StructKind::String | StructKind::ConstString => String::from("string"),
    }
}
