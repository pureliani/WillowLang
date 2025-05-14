use crate::ast::checked::checked_type::CheckedType;

pub fn is_signed(ty: &CheckedType) -> bool {
    use CheckedType::*;
    matches!(ty, I8 | I16 | I32 | I64 | ISize | F32 | F64)
}
