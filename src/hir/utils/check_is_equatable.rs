use crate::hir::types::checked_type::{StructKind, Type};

use super::numeric::is_integer;

pub fn check_is_equatable(left: &Type, right: &Type) -> bool {
    match (left, right) {
        (Type::Bool, Type::Bool) => true,
        (Type::Struct(struct_a), Type::Struct(struct_b)) => match (struct_a, struct_b) {
            (StructKind::String, StructKind::String) => true,
            (StructKind::String, StructKind::ConstString) => true,
            (StructKind::ConstString, StructKind::String) => true,
            (StructKind::ConstString, StructKind::ConstString) => true,
            (StructKind::Tag(tag_a), StructKind::Tag(tag_b)) => {
                todo!("Check if they have the same identifier and value type")
            }
            _ => false,
        },
        (a, b) if is_integer(a) && is_integer(b) => true,
        // TODO: add other kinds
        _ => false,
    }
}
