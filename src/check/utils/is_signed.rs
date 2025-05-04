use crate::ast::checked::checked_type::{Type, TypeKind};

pub fn is_signed(ty: &Type) -> bool {
    use TypeKind::*;
    matches!(ty.kind, I8 | I16 | I32 | I64 | ISize | F32 | F64)
}
