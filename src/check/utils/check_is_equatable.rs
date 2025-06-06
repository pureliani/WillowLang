use crate::ast::checked::checked_type::CheckedTypeKind;

use super::is_integer::is_integer;

pub fn check_is_equatable(left: &CheckedTypeKind, right: &CheckedTypeKind) -> bool {
    match (left, right) {
        (CheckedTypeKind::Bool { .. }, CheckedTypeKind::Bool { .. }) => true,
        (CheckedTypeKind::Char { .. }, CheckedTypeKind::Char { .. }) => true,
        (a, b) if is_integer(a) && is_integer(b) => true,
        (CheckedTypeKind::EnumDecl { decl: a, .. }, CheckedTypeKind::EnumDecl { decl: b, .. }) => a == b,
        (CheckedTypeKind::Union(a_items), CheckedTypeKind::Union(b_items)) => {
            a_items.iter().any(|a| b_items.iter().any(|b| check_is_equatable(a, b)))
        }
        (CheckedTypeKind::Union(items), other) | (other, CheckedTypeKind::Union(items)) => {
            items.iter().any(|item| check_is_equatable(item, other))
        }
        _ => false,
    }
}
