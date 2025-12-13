use crate::{
    ast::Span,
    hir::{
        errors::{SemanticError, SemanticErrorKind},
        types::{
            checked_declaration::TagType,
            checked_type::{StructKind, Type},
        },
        FunctionBuilder,
    },
};

impl FunctionBuilder {
    pub fn try_unify_types(
        &self,
        entries: &[(Type, Span)],
    ) -> Result<Type, SemanticError> {
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

            collected_tags.sort_by(|a, b| a.id.0.cmp(&b.id.0));

            if collected_tags.len() == 1 {
                return Ok(Type::Struct(StructKind::Tag(collected_tags.pop().unwrap())));
            }

            return Ok(Type::Struct(StructKind::Union {
                variants: collected_tags,
            }));
        }

        let (first_type, _) = &entries[0];
        for (ty, span) in entries.iter().skip(1) {
            if !self.check_is_assignable(ty, first_type) {
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
}
