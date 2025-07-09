use crate::hir_builder::types::checked_type::TypeKind;

pub fn is_integer(ty: &TypeKind) -> bool {
    use TypeKind::*;
    matches!(ty, I8 | I16 | I32 | I64 | U8 | U16 | U32 | U64 | ISize | USize)
}
