use crate::ast::checked::checked_type::{Type, TypeKind, TypeSpan};

pub fn union_of(types: &[Type]) -> Type {
    let mut union_items: Vec<Type> = vec![];

    for t in types {
        match &t.kind {
            TypeKind::Union(items) => {
                union_items.extend(items.clone());
            }
            _ => union_items.push(t.clone()),
        }
    }

    // TODO: somehow deduplicate union items

    Type {
        kind: TypeKind::Union(union_items),
        span: TypeSpan::None,
    }
}
