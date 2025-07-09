use std::collections::HashSet;

use crate::ast::{
    checked::checked_type::{Type, TypeKind},
    Span,
};

pub fn union_of(types: impl IntoIterator<Item = Type>, span: Span) -> Type {
    let mut union_items: HashSet<Type> = HashSet::new();

    for t in types {
        match t.kind {
            TypeKind::Union(items) => {
                for item in items {
                    if !matches!(item.kind, TypeKind::Void) {
                        union_items.insert(item);
                    }
                }
            }
            _ => {
                union_items.insert(t);
            }
        };
    }

    if union_items.is_empty() {
        return Type {
            kind: TypeKind::Void,
            span,
        };
    }

    if union_items.len() == 1 {
        return union_items.into_iter().next().unwrap();
    }

    Type {
        kind: TypeKind::Union(union_items),
        span,
    }
}

pub fn union_of_kinds(types: impl IntoIterator<Item = TypeKind>) -> TypeKind {
    let mut union_items: HashSet<Type> = HashSet::new();

    for t_rc in types {
        match &t_rc {
            TypeKind::Union(items) => {
                for item in items {
                    if !matches!(item.kind, TypeKind::Void) {
                        union_items.insert(item.clone());
                    }
                }
            }
            TypeKind::Unknown => {
                return TypeKind::Unknown;
            }
            TypeKind::Void => {}
            _ => {
                // A placeholder span is acceptable for internal calculations.
                union_items.insert(Type {
                    kind: t_rc.clone(),
                    span: crate::ast::Span {
                        // Using a dummy span
                        start: crate::ast::Position {
                            line: 0,
                            col: 0,
                            byte_offset: 0,
                        },
                        end: crate::ast::Position {
                            line: 0,
                            col: 0,
                            byte_offset: 0,
                        },
                    },
                });
            }
        };
    }

    if union_items.is_empty() {
        return TypeKind::Void;
    }

    if union_items.len() == 1 {
        return union_items.into_iter().next().unwrap().kind;
    }

    TypeKind::Union(union_items)
}
