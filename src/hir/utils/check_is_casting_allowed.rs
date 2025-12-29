use crate::hir::{
    types::checked_type::{PointerKind, Type},
    utils::numeric::{get_numeric_type_rank, is_float, is_integer, is_signed},
    FunctionBuilder,
};

impl FunctionBuilder {
    pub fn check_is_casting_allowed(
        &self,
        source_type: &Type,
        target_type: &Type,
    ) -> bool {
        match (&source_type, &target_type) {
            (st, tt)
                if is_integer(st)
                    && is_integer(tt)
                    && (is_signed(st) == is_signed(tt)) =>
            {
                get_numeric_type_rank(st) <= get_numeric_type_rank(tt)
            }
            (st, tt) if is_float(st) && is_float(tt) => {
                get_numeric_type_rank(st) <= get_numeric_type_rank(tt)
            }
            (st, tt) if is_integer(st) && is_float(tt) => true,

            (
                Type::Pointer { kind: src_kind, .. },
                Type::Pointer { kind: tgt_kind, .. },
            ) => match (src_kind, tgt_kind) {
                (PointerKind::Mut, PointerKind::Ref) => true,
                (PointerKind::Mut, PointerKind::Raw) => true,

                (PointerKind::Raw, PointerKind::Mut) => true,
                (PointerKind::Raw, PointerKind::Ref) => true,

                (PointerKind::Ref, PointerKind::Raw) => true,
                (PointerKind::Ref, PointerKind::Mut) => false,

                _ => false,
            },
            (Type::Pointer { .. }, t) if is_integer(t) => true,
            (s, Type::Pointer { .. }) if is_integer(s) => true,
            _ => false,
        }
    }
}
