use std::collections::HashSet;

use crate::ast::checked::checked_type::{CheckedType, CheckedTypeKind, TypeSpan};

pub fn union_of(types: impl Iterator<Item = CheckedType>) -> CheckedType {
    let mut union_items: HashSet<CheckedType> = HashSet::new();

    for t in types {
        match t.kind {
            CheckedTypeKind::Union(items) => {
                union_items.extend(items);
            }
            _ => {
                union_items.insert(t);
            }
        };
    }

    CheckedType {
        kind: CheckedTypeKind::Union(union_items),
        span: TypeSpan::None,
    }
}
