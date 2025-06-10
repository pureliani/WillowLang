use std::collections::HashSet;

use crate::ast::{
    checked::checked_type::{CheckedType, CheckedTypeKind},
    Span,
};

pub fn union_of(types: impl IntoIterator<Item = CheckedType>, span: Span) -> CheckedType {
    let mut union_items: HashSet<CheckedType> = HashSet::new();

    for t in types {
        match t.kind {
            CheckedTypeKind::Union(items) => {
                for item in items {
                    if !matches!(item.kind, CheckedTypeKind::Void) {
                        union_items.insert(item);
                    }
                }
            }
            _ => {
                union_items.insert(t);
            }
        };
    }

    CheckedType {
        kind: CheckedTypeKind::Union(union_items),
        span,
    }
}
