use crate::ast::checked::checked_type::{Type, TypeKind};

pub fn check_is_assignable(source_type: &Type, target_type: &Type) -> bool {
    use TypeKind::*;

    match (&source_type.kind, &target_type.kind) {
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
        (Union(left), Union(right)) => left.iter().all(|left_item| {
            right
                .iter()
                .any(|right_item| check_is_assignable(left_item, right_item))
        }),
        (GenericParam(left), GenericParam(right)) => {
            match (&left.constraint, &right.constraint) {
                (Some(left_constraint), Some(right_constraint)) => {
                    check_is_assignable(&left_constraint, &right_constraint)
                }
                // generics might be substituted with types that are incompatible, we have to take conservative approach
                // e.g
                // struct User {
                //   name: string
                // }
                //
                // let something = <A, B>(a: A, b: B) => {
                //   let x: A | null = null;
                //   x = b; // "b" should not be assignable to x
                // }
                _ => false,
            }
        }
        (Struct(left), Struct(right)) => todo!(),
        (
            Array {
                item_type: left_type,
                size: left_size,
            },
            Array {
                item_type: right_type,
                size: right_size,
            },
        ) => {
            let same_size = left_size == right_size;
            let assignable_types = check_is_assignable(&left_type, &right_type);

            same_size && assignable_types
        }
        (
            FnType {
                params: left_params,
                return_type: left_return_type,
                generic_params: left_generic_params,
            },
            FnType {
                params: right_params,
                return_type: right_return_type,
                generic_params: right_generic_params,
            },
        ) => todo!(),
        (Enum(left), Enum(right)) => {
            let same_name = left.identifier.name == right.identifier.name;
            let same_len = left.variants.len() == right.variants.len();

            false
        }
        (TypeAlias(left), right) => todo!(),
        _ => false,
    }
}
