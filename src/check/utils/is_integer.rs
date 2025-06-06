use crate::ast::checked::checked_type::CheckedTypeKind;

pub fn is_integer(ty: &CheckedTypeKind) -> bool {
    use CheckedTypeKind::*;
    matches!(ty, I8 | I16 | I32 | I64 | U8 | U16 | U32 | U64 | ISize | USize)
}
