use crate::ast::checked::checked_type::{CheckedType, CheckedTypeKind};

pub fn is_float(ty: &CheckedType) -> bool {
    use CheckedTypeKind::*;
    matches!(ty.kind, F32 | F64)
}
