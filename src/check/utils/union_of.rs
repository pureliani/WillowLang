use std::collections::HashSet;

use crate::ast::checked::checked_type::CheckedType;

pub fn union_of(types: impl IntoIterator<Item = CheckedType>) -> CheckedType {
    let mut union_items: HashSet<CheckedType> = HashSet::new();

    for t in types {
        match t {
            CheckedType::Union(items) => {
                union_items.extend(items);
            }
            _ => {
                union_items.insert(t);
            }
        };
    }

    CheckedType::Union(union_items)
}
