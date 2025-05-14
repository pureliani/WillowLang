use crate::ast::checked::checked_type::CheckedType;

pub fn is_float(ty: &CheckedType) -> bool {
    use CheckedType::*;
    matches!(ty, F32 | F64)
}
