use crate::hir::types::checked_type::TypeKind;

pub fn is_float(ty: &TypeKind) -> bool {
    use TypeKind::*;
    matches!(ty, F32 | F64)
}
