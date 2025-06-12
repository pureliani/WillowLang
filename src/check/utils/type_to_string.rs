use crate::{
    ast::checked::{
        checked_declaration::{CheckedFnType, CheckedGenericParam, CheckedParam},
        checked_type::{CheckedType, CheckedTypeKind},
    },
    compile::string_interner::{InternerId, StringInterner},
};

fn applied_type_args_to_string(type_args: &Vec<CheckedType>, string_interner: &StringInterner) -> String {
    if type_args.is_empty() {
        return "".to_string();
    }
    let joined_args = type_args
        .iter()
        .map(|arg_ty| type_to_string(&arg_ty.kind, string_interner))
        .collect::<Vec<String>>()
        .join(", ");
    format!("<{}>", joined_args)
}

fn identifier_to_string(id: InternerId, string_interner: &StringInterner) -> String {
    let identifier_name = string_interner.resolve(id).unwrap();

    identifier_name.to_owned()
}

fn generic_params_to_string(generic_params: &Vec<CheckedGenericParam>, string_interner: &StringInterner) -> String {
    if !generic_params.is_empty() {
        let joined = generic_params
            .iter()
            .map(|gp| {
                let name = identifier_to_string(gp.identifier.name, string_interner);

                match &gp.constraint {
                    Some(c) => {
                        format!("{}: {}", name, type_to_string(&c.kind, string_interner))
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

pub fn type_to_string(ty: &CheckedTypeKind, string_interner: &StringInterner) -> String {
    // TODO: add recursion detection and handling

    match ty {
        CheckedTypeKind::Void => String::from("void"),
        CheckedTypeKind::Null => String::from("null"),
        CheckedTypeKind::Bool => String::from("bool"),
        CheckedTypeKind::U8 => String::from("u8"),
        CheckedTypeKind::U16 => String::from("u16"),
        CheckedTypeKind::U32 => String::from("u32"),
        CheckedTypeKind::U64 => String::from("u64"),
        CheckedTypeKind::USize => String::from("usize"),
        CheckedTypeKind::ISize => String::from("isize"),
        CheckedTypeKind::I8 => String::from("i8"),
        CheckedTypeKind::I16 => String::from("i16"),
        CheckedTypeKind::I32 => String::from("i32"),
        CheckedTypeKind::I64 => String::from("i64"),
        CheckedTypeKind::F32 => String::from("f32"),
        CheckedTypeKind::F64 => String::from("f64"),
        CheckedTypeKind::Char => String::from("char"),
        CheckedTypeKind::Unknown => String::from("unknown"),
        CheckedTypeKind::StructDecl(decl) => {
            let decl = decl.borrow();

            let name = identifier_to_string(decl.identifier.name, string_interner);

            if decl.applied_type_args.len() > 0 {
                format!(
                    "{}{}",
                    name,
                    applied_type_args_to_string(&decl.applied_type_args, string_interner)
                )
            } else {
                let generics_str = generic_params_to_string(&decl.generic_params, string_interner);
                format!("{}{}", name, generics_str)
            }
        }
        CheckedTypeKind::EnumDecl(decl) => {
            let decl = decl.borrow();

            let name = identifier_to_string(decl.identifier.name, string_interner);

            name
        }
        CheckedTypeKind::GenericParam(CheckedGenericParam { identifier, .. }) => {
            let name = identifier_to_string(identifier.name, string_interner);

            name
        }
        CheckedTypeKind::FnType(CheckedFnType {
            params,
            return_type,
            generic_params,
            applied_type_args,
            span: _,
        }) => {
            let type_args_str = if applied_type_args.len() > 0 {
                applied_type_args_to_string(&applied_type_args, string_interner)
            } else {
                generic_params_to_string(&generic_params, string_interner)
            };

            let params_str = params
                .iter()
                .map(|p| type_to_string(&p.constraint.kind, string_interner))
                .collect::<Vec<String>>()
                .join(", ");

            let return_type_str = type_to_string(&return_type.kind, string_interner);

            format!("({}({}) => {})", type_args_str, params_str, return_type_str)
        }
        CheckedTypeKind::TypeAliasDecl(decl) => {
            let decl = decl.borrow();
            let name = identifier_to_string(decl.identifier.name, string_interner);

            if decl.applied_type_args.len() > 0 {
                format!(
                    "{}{}",
                    name,
                    applied_type_args_to_string(&decl.applied_type_args, string_interner)
                )
            } else {
                let generics_str = generic_params_to_string(&decl.generic_params, string_interner);
                format!("{}{}", name, generics_str)
            }
        }
        CheckedTypeKind::Union(hash_set) => hash_set
            .iter()
            .map(|t| type_to_string(&t.kind, string_interner))
            .collect::<Vec<String>>()
            .join(" | "),

        CheckedTypeKind::Array { item_type, size } => {
            format!("[{}; {}]", type_to_string(&item_type.kind, string_interner), size)
        }
    }
}
