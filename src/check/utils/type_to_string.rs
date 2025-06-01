use crate::{
    ast::{
        base::base_declaration::EnumDecl,
        checked::{
            checked_declaration::{
                CheckedGenericParam, CheckedGenericStructDecl, CheckedGenericTypeAliasDecl,
                CheckedParam, CheckedStructDecl, CheckedTypeAliasDecl,
            },
            checked_type::CheckedType,
        },
    },
    compile::string_interner::{InternerId, StringInterner},
};

fn identifier_to_string(id: InternerId, string_interner: &StringInterner) -> String {
    let identifier_name = string_interner.resolve(id).unwrap();

    identifier_name.to_owned()
}

fn param_to_string(param: &CheckedParam, string_interner: &StringInterner) -> String {
    let name = identifier_to_string(param.identifier.name, string_interner);
    let constraint = type_to_string(&param.constraint, string_interner);

    format!("{}: {}", name, constraint)
}

fn generic_params_to_string(
    generic_params: &Vec<CheckedGenericParam>,
    string_interner: &StringInterner,
) -> String {
    if !generic_params.is_empty() {
        let joined = generic_params
            .iter()
            .map(|gp| {
                let name = identifier_to_string(gp.identifier.name, string_interner);

                match &gp.constraint {
                    Some(c) => {
                        format!("{}: {}", name, type_to_string(c, string_interner))
                    }
                    None => {
                        format!("{}", name)
                    }
                }
            })
            .collect::<Vec<String>>()
            .join(", ");

        format!("<{}>", joined)
    } else {
        "".to_owned()
    }
}

pub fn type_to_string(ty: &CheckedType, string_interner: &StringInterner) -> String {
    match ty {
        CheckedType::Void => String::from("void"),
        CheckedType::Null => String::from("null"),
        CheckedType::Bool => String::from("bool"),
        CheckedType::U8 => String::from("u8"),
        CheckedType::U16 => String::from("u16"),
        CheckedType::U32 => String::from("u32"),
        CheckedType::U64 => String::from("u64"),
        CheckedType::USize => String::from("usize"),
        CheckedType::ISize => String::from("isize"),
        CheckedType::I8 => String::from("i8"),
        CheckedType::I16 => String::from("i16"),
        CheckedType::I32 => String::from("i32"),
        CheckedType::I64 => String::from("i64"),
        CheckedType::F32 => String::from("f32"),
        CheckedType::F64 => String::from("f64"),
        CheckedType::Char => String::from("char"),
        CheckedType::Unknown => String::from("unknown"),
        CheckedType::GenericStructDecl(CheckedGenericStructDecl {
            generic_params,
            identifier,
            ..
        }) => {
            let name = identifier_to_string(identifier.name, string_interner);
            let generic_params_str = generic_params_to_string(generic_params, string_interner);

            format!("{}{}", name, generic_params_str)
        }
        CheckedType::StructDecl(CheckedStructDecl { identifier, .. }) => {
            let name = identifier_to_string(identifier.name, string_interner);

            name
        }
        CheckedType::EnumDecl(EnumDecl { identifier, .. }) => {
            let name = identifier_to_string(identifier.name, string_interner);

            name
        }
        CheckedType::GenericParam(CheckedGenericParam { identifier, .. }) => {
            let name = identifier_to_string(identifier.name, string_interner);

            name
        }
        CheckedType::GenericFnType {
            params,
            return_type,
            generic_params,
        } => {
            let generic_params_str = generic_params_to_string(generic_params, string_interner);
            let params_str = {
                let joined = params
                    .iter()
                    .map(|p| param_to_string(p, string_interner))
                    .collect::<Vec<String>>()
                    .join(", ");

                format!("({})", joined)
            };
            let return_type_str = type_to_string(&return_type, string_interner);

            format!(
                "({}{} => {})",
                generic_params_str, params_str, return_type_str
            )
        }
        CheckedType::FnType {
            params,
            return_type,
        } => {
            let params_str = {
                let joined = params
                    .iter()
                    .map(|p| param_to_string(p, string_interner))
                    .collect::<Vec<String>>()
                    .join(", ");

                format!("({})", joined)
            };
            let return_type_str = type_to_string(&return_type, string_interner);

            format!("({} => {})", params_str, return_type_str)
        }
        CheckedType::GenericTypeAliasDecl(CheckedGenericTypeAliasDecl {
            generic_params,
            identifier,
            ..
        }) => {
            let name = identifier_to_string(identifier.name, string_interner);
            let generic_params_str = generic_params_to_string(generic_params, string_interner);

            format!("{}{}", name, generic_params_str)
        }
        CheckedType::TypeAliasDecl(CheckedTypeAliasDecl { identifier, .. }) => {
            let name = identifier_to_string(identifier.name, string_interner);

            name
        }
        CheckedType::Union(hash_set) => hash_set
            .iter()
            .map(|t| type_to_string(t, string_interner))
            .collect::<Vec<String>>()
            .join(" | "),

        CheckedType::Array { item_type, size } => {
            format!("[{}; {}]", type_to_string(item_type, string_interner), size)
        }
    }
}
