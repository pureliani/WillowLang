use crate::{
    ast::Span,
    compile::interner::TagId,
    hir::{
        errors::{SemanticError, SemanticErrorKind},
        types::{
            checked_declaration::TagType,
            checked_type::{StructKind, Type},
        },
        utils::check_is_assignable::check_is_assignable,
    },
};

pub fn try_unify_types(entries: &[(Type, Span)]) -> Result<Type, SemanticError> {
    if entries.is_empty() {
        return Ok(Type::Void);
    }

    let all_tags = entries.iter().all(|(t, _)| {
        matches!(
            t,
            Type::Struct(StructKind::Tag(_)) | Type::Struct(StructKind::Union { .. })
        )
    });

    if all_tags {
        let mut collected_tags: Vec<TagType> = Vec::new();
        for (ty, _) in entries {
            match ty {
                Type::Struct(StructKind::Tag(tag)) => {
                    if !collected_tags.contains(tag) {
                        collected_tags.push(tag.clone());
                    }
                }
                Type::Struct(StructKind::Union { variants }) => {
                    for tag in variants {
                        if !collected_tags.contains(tag) {
                            collected_tags.push(tag.clone());
                        }
                    }
                }
                _ => unreachable!(),
            }
        }

        return Ok(wrap_variants(collected_tags));
    }

    let (first_type, _) = &entries[0];
    for (ty, span) in entries.iter().skip(1) {
        if !check_is_assignable(ty, first_type) {
            return Err(SemanticError {
                span: *span,
                kind: SemanticErrorKind::TypeMismatch {
                    expected: first_type.clone(),
                    received: ty.clone(),
                },
            });
        }
    }

    Ok(first_type.clone())
}

pub fn subtract_types(base: &Type, to_remove: &[TagId]) -> Type {
    match base {
        Type::Pointer {
            constraint,
            narrowed_to,
        } => Type::Pointer {
            constraint: constraint.clone(),
            narrowed_to: Box::new(subtract_types(narrowed_to, to_remove)),
        },
        Type::Struct(StructKind::Union { variants }) => {
            let remaining: Vec<TagType> = variants
                .iter()
                .filter(|v| !to_remove.contains(&v.id))
                .cloned()
                .collect();

            wrap_variants(remaining)
        }
        Type::Struct(StructKind::Tag(t)) => {
            if to_remove.contains(&t.id) {
                Type::Void
            } else {
                base.clone()
            }
        }
        _ => base.clone(),
    }
}

pub fn intersect_types(base: &Type, allowed: &[TagId]) -> Type {
    match base {
        Type::Pointer {
            constraint,
            narrowed_to,
        } => Type::Pointer {
            constraint: constraint.clone(),
            narrowed_to: Box::new(intersect_types(narrowed_to, allowed)),
        },
        Type::Struct(StructKind::Union { variants }) => {
            let kept: Vec<TagType> = variants
                .iter()
                .filter(|v| allowed.contains(&v.id))
                .cloned()
                .collect();

            wrap_variants(kept)
        }
        Type::Struct(StructKind::Tag(t)) => {
            if allowed.contains(&t.id) {
                base.clone()
            } else {
                Type::Void
            }
        }
        Type::Unknown => Type::Unknown,
        _ => Type::Void,
    }
}

fn wrap_variants(mut variants: Vec<TagType>) -> Type {
    if variants.is_empty() {
        return Type::Void;
    }

    if variants.len() == 1 {
        return Type::Struct(StructKind::Tag(variants.pop().unwrap()));
    }

    variants.sort_by(|a, b| a.id.0.cmp(&b.id.0));

    Type::Struct(StructKind::Union { variants })
}
