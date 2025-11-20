use crate::hir::types::checked_type::{StructKind, Type};

use super::numeric::is_integer;

pub fn check_is_equatable(left: &Type, right: &Type) -> bool {
    match (left, right) {
        (Type::Bool, Type::Bool) => true,
        (Type::Struct(a), Type::Struct(b)) => match (a.kind(), b.kind()) {
            (StructKind::String, StructKind::String) => true,
            (StructKind::String, StructKind::ConstString) => true,
            (StructKind::ConstString, StructKind::String) => true,
            (StructKind::ConstString, StructKind::ConstString) => true,
            (StructKind::Tag, StructKind::Tag) => {
                todo!("Check if they have the same identifier and value type")
            }
            _ => false,
        },
        (a, b) if is_integer(a) && is_integer(b) => true,
        // TODO: add other kinds
        _ => false,
    }
}
