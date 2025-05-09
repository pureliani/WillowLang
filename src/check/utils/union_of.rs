use crate::ast::checked::checked_type::{CheckedType, CheckedTypeKind, TypeSpan};

pub fn union_of(types: &[CheckedType]) -> CheckedType {
    let mut union_items: Vec<CheckedType> = vec![];

    for t in types {
        match &t.kind {
            CheckedTypeKind::Union(items) => {
                union_items.extend(items.clone());
            }
            _ => union_items.push(t.clone()),
        }
    }

    // TODO: somehow deduplicate union items

    CheckedType {
        kind: CheckedTypeKind::Union(union_items),
        span: TypeSpan::None,
    }
}
