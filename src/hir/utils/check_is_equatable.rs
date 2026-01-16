use super::check_is_assignable::check_is_assignable;
use super::numeric::is_integer;
use crate::hir::types::checked_type::{StructKind, Type};

pub fn check_is_equatable(left: &Type, right: &Type) -> bool {
    if check_is_assignable(left, right) || check_is_assignable(right, left) {
        // Filter out types that shouldn't be compared even if they match
        return !matches!(left, Type::Void | Type::Fn(_) | Type::Unknown);
    }

    if is_integer(left) && is_integer(right) {
        return true;
    }

    if matches!(left, Type::Struct(StructKind::String))
        && matches!(right, Type::Struct(StructKind::String))
    {
        return true;
    }

    if let (Type::Pointer(l), Type::Pointer(r)) = (left, right) {
        return l == r;
    }

    false
}
