use crate::ast::checked::checked_type::{CheckedType, CheckedTypeKind};

pub fn is_integer(ty: &CheckedType) -> bool {
    use CheckedTypeKind::*;
    matches!(
        ty.kind,
        I8 | I16 | I32 | I64 | U8 | U16 | U32 | U64 | ISize | USize
    )
}
