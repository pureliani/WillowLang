use crate::ast::checked::checked_type::{CheckedTypeX, CheckedType};

pub fn is_integer(ty: &CheckedTypeX) -> bool {
    use CheckedType::*;
    matches!(
        ty.kind,
        I8 | I16 | I32 | I64 | U8 | U16 | U32 | U64 | ISize | USize
    )
}
