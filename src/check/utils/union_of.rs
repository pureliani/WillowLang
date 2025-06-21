use std::{collections::HashSet, rc::Rc};

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

    if union_items.is_empty() {
        return CheckedType {
            kind: CheckedTypeKind::Void,
            span,
        };
    }

    if union_items.len() == 1 {
        return union_items.into_iter().next().unwrap();
    }

    CheckedType {
        kind: CheckedTypeKind::Union(union_items),
        span,
    }
}

pub fn union_of_kinds(types: impl IntoIterator<Item = Rc<CheckedTypeKind>>) -> Rc<CheckedTypeKind> {
    let mut union_items: HashSet<CheckedType> = HashSet::new();

    for t_rc in types {
        match &*t_rc {
            CheckedTypeKind::Union(items) => {
                for item in items {
                    if !matches!(item.kind, CheckedTypeKind::Void) {
                        union_items.insert(item.clone());
                    }
                }
            }
            CheckedTypeKind::Unknown => {
                return Rc::new(CheckedTypeKind::Unknown);
            }
            CheckedTypeKind::Void => {}
            _ => {
                // A placeholder span is acceptable for internal calculations.
                union_items.insert(CheckedType {
                    kind: (*t_rc).clone(),
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
        return Rc::new(CheckedTypeKind::Void);
    }

    if union_items.len() == 1 {
        return Rc::new(union_items.into_iter().next().unwrap().kind);
    }

    Rc::new(CheckedTypeKind::Union(union_items))
}
