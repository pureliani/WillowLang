use crate::hir::types::checked_type::{StructLayoutKind, Type};

use super::numeric::is_integer;

pub fn check_is_equatable(left: &Type, right: &Type) -> bool {
    match (left, right) {
        (Type::Bool, Type::Bool) => true,
        (Type::Struct(a), Type::Struct(b)) => match (a.kind(), b.kind()) {
            (StructLayoutKind::String, StructLayoutKind::String) => true,
            (StructLayoutKind::String, StructLayoutKind::ConstString) => true,
            (StructLayoutKind::ConstString, StructLayoutKind::String) => true,
            (StructLayoutKind::ConstString, StructLayoutKind::ConstString) => true,
            (StructLayoutKind::Tag, StructLayoutKind::Tag) => {
                todo!("Check if they have the same identifier and value type")
            }
            _ => false,
        },
        (a, b) if is_integer(a) && is_integer(b) => true,
        // TODO: add other kinds
        _ => false,
    }
}
