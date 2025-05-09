use crate::ast::checked::checked_type::{CheckedType, CheckedTypeKind};

pub fn is_signed(ty: &CheckedType) -> bool {
    use CheckedTypeKind::*;
    matches!(ty.kind, I8 | I16 | I32 | I64 | ISize | F32 | F64)
}
