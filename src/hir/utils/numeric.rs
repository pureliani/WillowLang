use crate::hir::types::checked_type::TypeKind;

pub fn get_numeric_type_rank(ty: &TypeKind) -> i32 {
    use TypeKind::*;
    match &ty {
        I8 | U8 => 1,
        I16 | U16 => 2,
        I32 | U32 | ISize | USize => 3,
        I64 | U64 => 4,
        F32 => 5,
        F64 => 6,
        _ => 0,
    }
}

pub fn is_float(ty: &TypeKind) -> bool {
    use TypeKind::*;
    matches!(ty, F32 | F64)
}

pub fn is_integer(ty: &TypeKind) -> bool {
    use TypeKind::*;
    matches!(
        ty,
        I8 | I16 | I32 | I64 | U8 | U16 | U32 | U64 | ISize | USize
    )
}

pub fn is_signed(ty: &TypeKind) -> bool {
    use TypeKind::*;
    matches!(ty, I8 | I16 | I32 | I64 | ISize | F32 | F64)
}
