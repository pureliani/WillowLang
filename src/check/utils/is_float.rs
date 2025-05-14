use crate::ast::checked::checked_type::{CheckedTypeX, CheckedType};

pub fn is_float(ty: &CheckedTypeX) -> bool {
    use CheckedType::*;
    matches!(ty.kind, F32 | F64)
}
