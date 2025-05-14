use crate::ast::checked::checked_type::CheckedType;

pub fn check_is_assignable(source_type: &CheckedType, target_type: &CheckedType) -> bool {
    use CheckedType::*;

    match (&source_type, &target_type) {
        (I8, I8)
        | (I16, I16)
        | (I32, I32)
        | (I64, I64)
        | (ISize, ISize)
        | (U8, U8)
        | (U16, U16)
        | (U32, U32)
        | (U64, U64)
        | (USize, USize)
        | (F32, F32)
        | (F64, F64)
        | (Char, Char)
        | (Bool, Bool)
        | (Null, Null)
        | (Void, Void)
        | (Unknown, _) => true,
        (Union(source), Union(target)) => source.iter().all(|source_item| {
            target
                .iter()
                .any(|target_item| check_is_assignable(source_item, target_item))
        }),
        (GenericParam(source), GenericParam(target)) => {
            match (&source.constraint, &target.constraint) {
                (None, None) => true,
                (Some(_), None) => true,
                (None, Some(_)) => false,
                (Some(left_constraint), Some(right_constraint)) => {
                    check_is_assignable(&left_constraint, &right_constraint)
                }
            }
        }
        (StructDecl(source), StructDecl(target)) => {
            todo!()
        }
        (
            Array {
                item_type: source_type,
                size: source_size,
            },
            Array {
                item_type: target_type,
                size: target_size,
            },
        ) => {
            let same_size = source_size == target_size;
            let assignable_types = check_is_assignable(&source_type, &target_type);

            same_size && assignable_types
        }
        (
            FnType {
                params: source_params,
                return_type: source_return_type,
            },
            FnType {
                params: target_params,
                return_type: target_return_type,
            },
        ) => {
            todo!()
        }
        (
            GenericFnType {
                params: source_params,
                return_type: source_return_type,
                generic_params: source_generic_params,
            },
            GenericFnType {
                params: target_params,
                return_type: target_return_type,
                generic_params: target_generic_params,
            },
        ) => todo!(),
        (Enum(source), Enum(target)) => source == target,
        // TODO: add type alias handling
        _ => false,
    }
}
