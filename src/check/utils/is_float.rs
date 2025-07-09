use crate::ast::checked::checked_type::TypeKind;

pub fn is_float(ty: &TypeKind) -> bool {
    use TypeKind::*;
    matches!(ty, F32 | F64)
}
