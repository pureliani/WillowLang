use std::collections::HashSet;

use crate::ast::checked::checked_type::{CheckedTypeX, CheckedType, TypeSpan};

pub fn union_of(types: impl Iterator<Item = CheckedTypeX>) -> CheckedTypeX {
    let mut union_items: HashSet<CheckedTypeX> = HashSet::new();

    for t in types {
        match t.kind {
            CheckedType::Union(items) => {
                union_items.extend(items);
            }
            _ => {
                union_items.insert(t);
            }
        };
    }

    CheckedTypeX {
        kind: CheckedType::Union(union_items),
        span: TypeSpan::None,
    }
}
