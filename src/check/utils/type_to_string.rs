use crate::{
    ast::checked::{
        checked_declaration::{CheckedFnType, CheckedGenericParam},
        checked_type::{Type, TypeKind},
    },
    compile::string_interner::{InternerId, StringInterner},
};

fn applied_type_args_to_string(type_args: &Vec<Type>, string_interner: &StringInterner) -> String {
    if type_args.is_empty() {
        return "".to_string();
    }
    let joined_args = type_args
        .iter()
        .map(|arg_ty| type_to_string_recursive(&arg_ty.kind, string_interner, false))
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
                        format!("{}: {}", name, type_to_string_recursive(&c.kind, string_interner, false))
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

pub fn type_to_string(ty: &TypeKind, string_interner: &StringInterner) -> String {
    type_to_string_recursive(ty, string_interner, false)
}

pub fn type_to_string_recursive(ty: &TypeKind, string_interner: &StringInterner, is_union_context: bool) -> String {
    // TODO: add recursion detection and handling

    match ty {
        TypeKind::Void => String::from("void"),
        TypeKind::Null => String::from("null"),
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
        TypeKind::Char => String::from("char"),
        TypeKind::Unknown => String::from("unknown"),
        TypeKind::Struct(params) => {
            let params_str = params
                .iter()
                .map(|p| {
                    format!(
                        "{}: {}",
                        identifier_to_string(p.identifier.name, string_interner),
                        type_to_string_recursive(&p.constraint.kind, string_interner, false)
                    )
                })
                .collect::<Vec<String>>()
                .join(",\n");

            format!("{{\n{}\n}}", params_str)
        }
        TypeKind::GenericParam(CheckedGenericParam { identifier, .. }) => {
            let name = identifier_to_string(identifier.name, string_interner);

            name
        }
        TypeKind::FnType(CheckedFnType {
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
                .map(|p| {
                    format!(
                        "{}: {}",
                        identifier_to_string(p.identifier.name, string_interner),
                        type_to_string_recursive(&p.constraint.kind, string_interner, false)
                    )
                })
                .collect::<Vec<String>>()
                .join(", ");

            let return_type_str = type_to_string_recursive(&return_type.kind, string_interner, false);
            let fn_str = format!("{}({}) => {}", type_args_str, params_str, return_type_str);

            if is_union_context {
                format!("({})", fn_str)
            } else {
                fn_str
            }
        }
        TypeKind::TypeAliasDecl(decl) => {
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
        TypeKind::Union(hash_set) => hash_set
            .iter()
            .map(|t| type_to_string_recursive(&t.kind, string_interner, true))
            .collect::<Vec<String>>()
            .join(" | "),
        TypeKind::Array { item_type, size } => {
            format!(
                "[{}; {}]",
                type_to_string_recursive(&item_type.kind, string_interner, false),
                size
            )
        }
    }
}
