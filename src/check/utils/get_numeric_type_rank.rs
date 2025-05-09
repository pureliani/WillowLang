use crate::ast::checked::checked_type::{CheckedType, CheckedTypeKind};

pub fn get_numeric_type_rank(ty: &CheckedType) -> i32 {
    use CheckedTypeKind::*;
    match &ty.kind {
        I8 | U8 => 1,
        I16 | U16 => 2,
        I32 | U32 | ISize | USize => 3,
        I64 | U64 => 4,
        F32 => 5,
        F64 => 6,
        _ => 0,
    }
}
