use std::collections::HashSet;

use crate::{
    compile::interner::{Interners, StringId},
    hir::types::{
        checked_declaration::{FnType, TagType},
        checked_type::{StructKind, Type},
    },
    tokenize::TokenKind,
};

pub fn token_kind_to_string(kind: &TokenKind, interners: &Interners) -> String {
    match kind {
        TokenKind::Identifier(id) => interners.string_interner.resolve(*id).to_string(),
        TokenKind::Punctuation(punctuation_kind) => punctuation_kind.to_string(),
        TokenKind::Keyword(keyword_kind) => keyword_kind.to_string(),
        TokenKind::String(value) => value.to_owned(),
        TokenKind::Number(number_kind) => number_kind.to_string(),
        TokenKind::Doc(value) => format!("---\n{}\n---", value),
    }
}

fn identifier_to_string(id: StringId, interners: &Interners) -> String {
    interners.string_interner.resolve(id)
}

fn tag_type_to_string(
    tag: &TagType,
    interners: &Interners,
    visited_set: &mut HashSet<Type>,
) -> String {
    let name_string_id = interners.tag_interner.resolve(tag.id);
    let name = interners.string_interner.resolve(name_string_id);
    let value_str = tag
        .value_type
        .as_ref()
        .map(|v| format!("({})", type_to_string_recursive(v, interners, visited_set)))
        .unwrap_or_default();

    format!("#{0}{1}", name, value_str)
}

pub fn type_to_string(ty: &Type, interners: &Interners) -> String {
    let mut visited_set = HashSet::new();
    type_to_string_recursive(ty, interners, &mut visited_set)
}

pub fn type_to_string_recursive(
    ty: &Type,
    interners: &Interners,
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
        Type::Struct(s) => struct_to_string(s, interners, visited_set),
        Type::Fn(fn_type) => fn_signature_to_string(fn_type, interners, visited_set),
        Type::Pointer(to) => type_to_string_recursive(to, interners, visited_set),
        Type::Buffer { size, alignment } => {
            format!("Buffer(size={}, align={})", size, alignment)
        }
    };

    visited_set.remove(ty);

    result
}

fn fn_signature_to_string(
    fn_type: &FnType,
    interners: &Interners,
    visited_set: &mut HashSet<Type>,
) -> String {
    let params_str = fn_type
        .params
        .iter()
        .map(|p| {
            format!(
                "{}: {}",
                identifier_to_string(p.identifier.name, interners),
                type_to_string_recursive(&p.ty, interners, visited_set)
            )
        })
        .collect::<Vec<String>>()
        .join(", ");

    let return_type_str =
        type_to_string_recursive(&fn_type.return_type, interners, visited_set);

    format!("fn({}): {}", params_str, return_type_str)
}

fn struct_to_string(
    s: &StructKind,
    interners: &Interners,
    visited_set: &mut HashSet<Type>,
) -> String {
    match s {
        StructKind::UserDefined(fields) => {
            let fields_str = fields
                .iter()
                .map(|f| {
                    format!(
                        "{}: {}",
                        identifier_to_string(f.identifier.name, interners),
                        type_to_string_recursive(&f.ty, interners, visited_set)
                    )
                })
                .collect::<Vec<String>>()
                .join(", ");
            format!("{{ {} }}", fields_str)
        }
        StructKind::Tag(tag_type) => tag_type_to_string(tag_type, interners, visited_set),
        StructKind::Union { variants } => {
            if variants.is_empty() {
                return String::from("<empty union>");
            }
            let variants_str = variants
                .iter()
                .map(|tag| tag_type_to_string(tag, interners, visited_set))
                .collect::<Vec<String>>()
                .join(" | ");

            variants_str
        }
        StructKind::List(item_type) => {
            let elem_type_str =
                type_to_string_recursive(item_type, interners, visited_set);

            format!("{}[]", elem_type_str)
        }
        StructKind::String => String::from("string"),
    }
}
