use crate::ast::checked::checked_type::{CheckedTypeX, CheckedType};

pub fn is_signed(ty: &CheckedTypeX) -> bool {
    use CheckedType::*;
    matches!(ty.kind, I8 | I16 | I32 | I64 | ISize | F32 | F64)
}
