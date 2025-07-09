use crate::hir_builder::types::checked_type::TypeKind;

pub fn get_numeric_type_rank(ty: &TypeKind) -> i32 {
    use TypeKind::*;
    match &ty {
        I8 { .. } | U8 { .. } => 1,
        I16 { .. } | U16 { .. } => 2,
        I32 { .. } | U32 { .. } | ISize { .. } | USize { .. } => 3,
        I64 { .. } | U64 { .. } => 4,
        F32 { .. } => 5,
        F64 { .. } => 6,
        _ => 0,
    }
}
