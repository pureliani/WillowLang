use crate::hir::types::checked_type::TypeKind;

pub fn is_signed(ty: &TypeKind) -> bool {
    use TypeKind::*;
    matches!(ty, I8 | I16 | I32 | I64 | ISize | F32 | F64)
}
