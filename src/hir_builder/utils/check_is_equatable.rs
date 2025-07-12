use crate::hir_builder::types::checked_type::TypeKind;

use super::is_integer::is_integer;

pub fn check_is_equatable(left: &TypeKind, right: &TypeKind) -> bool {
    match (left, right) {
        (TypeKind::Bool, TypeKind::Bool) => true,
        (TypeKind::Char, TypeKind::Char) => true,
        (a, b) if is_integer(a) && is_integer(b) => true,
        // TODO: add other kinds
        _ => false,
    }
}
