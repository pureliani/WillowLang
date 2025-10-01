use crate::hir::{
    types::checked_type::{PointerKind, Type, TypeKind},
    utils::{get_numeric_type_rank::get_numeric_type_rank, is_float::is_float, is_integer::is_integer, is_signed::is_signed},
    FunctionBuilder,
};

impl FunctionBuilder {
    pub fn check_is_casting_allowed(&self, source_type: &Type, target_type: &Type) -> bool {
        match (&source_type.kind, &target_type.kind) {
            (st, tt) if is_integer(st) && is_integer(tt) && (is_signed(st) == is_signed(tt)) => {
                get_numeric_type_rank(st) <= get_numeric_type_rank(tt)
            }
            (st, tt) if is_float(st) && is_float(tt) => get_numeric_type_rank(st) <= get_numeric_type_rank(tt),
            (st, tt) if is_integer(st) && is_float(tt) => true,
            (
                TypeKind::Pointer {
                    kind: source_kind,
                    value_type: source_value_type,
                },
                TypeKind::Pointer {
                    kind: target_kind,
                    value_type: target_value_type,
                },
            ) => {
                if !self.check_is_assignable(source_value_type, target_value_type) {
                    return false;
                }

                match (source_kind, target_kind) {
                    (PointerKind::Raw, PointerKind::SharedBorrow) => true,
                    (PointerKind::Raw, PointerKind::MutableBorrow) => true,
                    (PointerKind::MutableBorrow, PointerKind::SharedBorrow) => true,
                    (PointerKind::SharedBorrow, PointerKind::Raw) => true,
                    (PointerKind::MutableBorrow, PointerKind::Raw) => true,
                    (a, b) => a == b,
                }
            }
            _ => false,
        }
    }
}
