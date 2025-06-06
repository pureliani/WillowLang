use crate::ast::checked::checked_type::CheckedTypeKind;

pub fn is_signed(ty: &CheckedTypeKind) -> bool {
    use CheckedTypeKind::*;
    matches!(ty, I8 | I16 | I32 | I64 | ISize | F32 | F64)
}
