use crate::ast::checked::checked_type::{Type, TypeKind};

pub fn is_float(ty: &Type) -> bool {
    use TypeKind::*;
    matches!(ty.kind, F32 | F64)
}
