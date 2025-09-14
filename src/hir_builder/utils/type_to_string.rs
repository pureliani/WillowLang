use crate::{
    compile::string_interner::{InternerId, StringInterner},
    hir_builder::types::{checked_declaration::CheckedFnType, checked_type::TypeKind},
};

fn identifier_to_string(id: InternerId, string_interner: &StringInterner) -> String {
    let identifier_name = string_interner.resolve(id);

    identifier_name.to_owned()
}

pub fn type_to_string(ty: &TypeKind, string_interner: &StringInterner) -> String {
    type_to_string_recursive(ty, string_interner)
}

pub fn type_to_string_recursive(ty: &TypeKind, string_interner: &StringInterner) -> String {
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
        TypeKind::Struct(decl) => {
            let fields = decl
                .fields
                .iter()
                .map(|p| {
                    format!(
                        "{}: {}",
                        identifier_to_string(p.identifier.name, string_interner),
                        type_to_string_recursive(&p.constraint.kind, string_interner)
                    )
                })
                .collect::<Vec<String>>()
                .join(",\n");

            format!(
                "struct {} {{\n{}\n}}",
                identifier_to_string(decl.identifier.name, string_interner),
                fields
            )
        }
        TypeKind::FnType(CheckedFnType {
            params,
            return_type,
            span: _,
        }) => {
            let params_str = params
                .iter()
                .map(|p| {
                    format!(
                        "{}: {}",
                        identifier_to_string(p.identifier.name, string_interner),
                        type_to_string_recursive(&p.constraint.kind, string_interner)
                    )
                })
                .collect::<Vec<String>>()
                .join(", ");

            let return_type_str = type_to_string_recursive(&return_type.kind, string_interner);
            let fn_str = format!("fn ({}): {}", params_str, return_type_str);

            fn_str
        }
        TypeKind::TypeAliasDecl(decl) => {
            let name = identifier_to_string(decl.identifier.name, string_interner);

            let value = type_to_string_recursive(&decl.value.kind, string_interner);

            format!("type {} = {};", name, value)
        }
        TypeKind::List(item_type) => {
            format!("{}[]", type_to_string_recursive(&item_type.kind, string_interner,))
        }
        TypeKind::Pointer(ty) => format!("ptr<{}>", type_to_string_recursive(&ty.kind, string_interner,)),
        TypeKind::Union(decl) => {
            let variants = decl
                .variants
                .iter()
                .map(|v| match &v.payload {
                    Some(pt) => {
                        format!(
                            "{}({})",
                            identifier_to_string(v.name.name, string_interner),
                            type_to_string(&pt.kind, string_interner)
                        )
                    }
                    None => {
                        format!("{}", identifier_to_string(v.name.name, string_interner))
                    }
                })
                .collect::<Vec<String>>()
                .join(",\n");

            format!(
                "union {} {{\n{}\n}}",
                identifier_to_string(decl.identifier.name, string_interner),
                variants
            )
        }
    }
}
