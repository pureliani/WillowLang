use std::{collections::HashSet, sync::Arc};

use crate::{
    compile::interner::{InternerId, SharedStringInterner},
    hir::types::{
        checked_declaration::{CheckedFnType, CheckedTagType},
        checked_type::TypeKind,
    },
};

fn identifier_to_string(
    id: InternerId,
    string_interner: Arc<SharedStringInterner>,
) -> String {
    let identifier_name = string_interner.resolve(id);

    identifier_name.to_owned()
}

fn tag_type_to_string(
    tag: &CheckedTagType,
    string_interner: Arc<SharedStringInterner>,
    visited_set: &mut HashSet<TypeKind>,
) -> String {
    let value_type = tag
        .value_type
        .as_ref()
        .map(|t| {
            format!(
                "({})",
                type_to_string_recursive(&t.kind, string_interner.clone(), visited_set)
            )
        })
        .unwrap_or("".to_string());

    format!(
        "#{}{}",
        identifier_to_string(tag.identifier.name, string_interner),
        value_type
    )
}

pub fn type_to_string(
    ty: &TypeKind,
    string_interner: Arc<SharedStringInterner>,
) -> String {
    let mut visited_set = HashSet::new();
    type_to_string_recursive(ty, string_interner, &mut visited_set)
}

pub fn type_to_string_recursive(
    ty: &TypeKind,
    string_interner: Arc<SharedStringInterner>,
    visited_set: &mut HashSet<TypeKind>,
) -> String {
    // TODO: add recursion detection and handling

    match ty {
        TypeKind::Void => String::from("void"),
        TypeKind::Bool => String::from("bool"),
        TypeKind::U8 => String::from("u8"),
        TypeKind::U16 => String::from("u16"),
        TypeKind::U32 => String::from("u32"),
        TypeKind::U64 => String::from("u64"),
        TypeKind::USize => String::from("usize"),
        TypeKind::ISize => String::from("isize"),
        TypeKind::I8 => String::from("i8"),
        TypeKind::I16 => String::from("i16"),
        TypeKind::I32 => String::from("i32"),
        TypeKind::I64 => String::from("i64"),
        TypeKind::F32 => String::from("f32"),
        TypeKind::F64 => String::from("f64"),
        TypeKind::String => String::from("string"),
        TypeKind::Unknown => String::from("unknown"),
        TypeKind::Struct(fields) => {
            let fields = fields
                .iter()
                .map(|field| {
                    format!(
                        "{}: {}",
                        identifier_to_string(
                            field.identifier.name,
                            string_interner.clone()
                        ),
                        type_to_string_recursive(
                            &field.ty.kind,
                            string_interner.clone(),
                            visited_set
                        )
                    )
                })
                .collect::<Vec<String>>()
                .join(",\n");

            format!("{{\n{}\n}}", fields)
        }
        TypeKind::FnType(CheckedFnType {
            params,
            return_type,
        }) => {
            let params_str = params
                .iter()
                .map(|p| {
                    format!(
                        "{}: {}",
                        identifier_to_string(p.identifier.name, string_interner.clone()),
                        type_to_string_recursive(
                            &p.ty.kind,
                            string_interner.clone(),
                            visited_set
                        )
                    )
                })
                .collect::<Vec<String>>()
                .join(", ");

            let return_type_str =
                type_to_string_recursive(&return_type.kind, string_interner, visited_set);
            let fn_str = format!("fn ({}): {}", params_str, return_type_str);

            fn_str
        }
        TypeKind::TypeAliasDecl(decl) => {
            let name = identifier_to_string(
                decl.read().unwrap().identifier.name,
                string_interner.clone(),
            );

            let value = type_to_string_recursive(
                &decl.read().unwrap().value.kind,
                string_interner.clone(),
                visited_set,
            );

            format!("type {} = {};", name, value)
        }
        TypeKind::Pointer(ty) => format!(
            "ptr<{}>",
            type_to_string_recursive(&ty.kind, string_interner, visited_set)
        ),
        TypeKind::Union(checked_tag_types) => {
            let variants = checked_tag_types
                .iter()
                .map(|tag| tag_type_to_string(tag, string_interner.clone(), visited_set))
                .collect::<Vec<String>>()
                .join(" | ");

            variants
        }
        TypeKind::List(item_type) => {
            let result =
                type_to_string_recursive(&item_type.kind, string_interner, visited_set);

            format!("{}[]", result)
        }
        TypeKind::Tag(checked_tag_type) => {
            tag_type_to_string(checked_tag_type, string_interner, visited_set)
        }
    }
}
