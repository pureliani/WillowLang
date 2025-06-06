use crate::ast::checked::checked_type::CheckedTypeKind;

pub fn is_float(ty: &CheckedTypeKind) -> bool {
    use CheckedTypeKind::*;
    matches!(ty, F32 | F64)
}
