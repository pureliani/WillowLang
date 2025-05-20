use crate::ast::checked::checked_type::CheckedType;

use super::is_integer::is_integer;

pub fn check_is_equatable(left: &CheckedType, right: &CheckedType) -> bool {
    match (left, right) {
        (CheckedType::Bool, CheckedType::Bool) => true,
        (CheckedType::Char, CheckedType::Char) => true,
        (a, b) if is_integer(a) && is_integer(b) => true,
        (CheckedType::Enum(a), CheckedType::Enum(b)) => a == b,

        (CheckedType::Union(a_items), CheckedType::Union(b_items)) => a_items
            .iter()
            .any(|a| b_items.iter().any(|b| check_is_equatable(a, b))),
        (CheckedType::Union(items), other) | (other, CheckedType::Union(items)) => {
            items.iter().any(|item| check_is_equatable(item, other))
        }
        _ => false,
    }
}
