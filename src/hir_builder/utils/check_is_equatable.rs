use crate::hir_builder::types::checked_type::TypeKind;

use super::is_integer::is_integer;

pub fn check_is_equatable(left: &TypeKind, right: &TypeKind) -> bool {
    match (left, right) {
        (TypeKind::Bool, TypeKind::Bool) => true,
        (TypeKind::Char, TypeKind::Char) => true,
        (a, b) if is_integer(a) && is_integer(b) => true,
        (TypeKind::Union(a_items), TypeKind::Union(b_items)) => a_items
            .iter()
            .any(|a| b_items.iter().any(|b| check_is_equatable(&a.kind, &b.kind))),
        (TypeKind::Union(items), other) | (other, TypeKind::Union(items)) => {
            items.iter().any(|item| check_is_equatable(&item.kind, other))
        }
        _ => false,
    }
}
