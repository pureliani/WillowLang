use crate::ast::checked::checked_type::{Type, TypeKind};

pub fn is_integer(ty: &Type) -> bool {
    use TypeKind::*;
    matches!(
        ty.kind,
        I8 | I16 | I32 | I64 | U8 | U16 | U32 | U64 | ISize | USize
    )
}
