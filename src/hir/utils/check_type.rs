use crate::{
    ast::{
        decl::Param,
        type_annotation::{TagAnnotation, TypeAnnotation, TypeAnnotationKind},
        IdentifierNode, Span,
    },
    hir::{
        cfg::CheckedDeclaration,
        errors::{SemanticError, SemanticErrorKind},
        types::{
            checked_declaration::{CheckedFnType, CheckedParam, CheckedTagType},
            checked_type::{Type, TypeKind},
        },
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn check_params(
        &mut self,
        ctx: &mut HIRContext,
        params: &Vec<Param>,
    ) -> Vec<CheckedParam> {
        params
            .into_iter()
            .map(|p| CheckedParam {
                ty: self.check_type_annotation(ctx, &p.constraint),
                identifier: p.identifier,
            })
            .collect()
    }

    pub fn check_type_identifier_annotation(
        &mut self,
        ctx: &mut HIRContext,
        id: IdentifierNode,
        span: Span,
    ) -> Result<TypeKind, SemanticError> {
        ctx.module_builder
            .scope_lookup(id.name)
            .map(|entry| match entry {
                CheckedDeclaration::TypeAlias(decl) => {
                    Ok(TypeKind::TypeAliasDecl(decl.clone()))
                }
                CheckedDeclaration::Function(_) => Err(SemanticError {
                    kind: SemanticErrorKind::CannotUseFunctionDeclarationAsType,
                    span,
                }),
                CheckedDeclaration::Var(_) => Err(SemanticError {
                    kind: SemanticErrorKind::CannotUseVariableDeclarationAsType,
                    span,
                }),
            })
            .unwrap_or_else(|| {
                Err(SemanticError {
                    kind: SemanticErrorKind::UndeclaredType(id),
                    span,
                })
            })
    }

    pub fn check_type_annotation(
        &mut self,
        ctx: &mut HIRContext,
        annotation: &TypeAnnotation,
    ) -> Type {
        let kind = match &annotation.kind {
            TypeAnnotationKind::Void => TypeKind::Void,
            TypeAnnotationKind::Bool => TypeKind::Bool,
            TypeAnnotationKind::U8 => TypeKind::U8,
            TypeAnnotationKind::U16 => TypeKind::U16,
            TypeAnnotationKind::U32 => TypeKind::U32,
            TypeAnnotationKind::U64 => TypeKind::U64,
            TypeAnnotationKind::USize => TypeKind::USize,
            TypeAnnotationKind::ISize => TypeKind::ISize,
            TypeAnnotationKind::I8 => TypeKind::I8,
            TypeAnnotationKind::I16 => TypeKind::I16,
            TypeAnnotationKind::I32 => TypeKind::I32,
            TypeAnnotationKind::I64 => TypeKind::I64,
            TypeAnnotationKind::F32 => TypeKind::F32,
            TypeAnnotationKind::F64 => TypeKind::F64,
            TypeAnnotationKind::String => TypeKind::String,
            TypeAnnotationKind::Identifier(id) => {
                match self.check_type_identifier_annotation(ctx, *id, annotation.span) {
                    Ok(t) => t,
                    Err(error) => {
                        ctx.program_builder.errors.push(error);
                        TypeKind::Unknown
                    }
                }
            }
            TypeAnnotationKind::FnType {
                params,
                return_type,
            } => {
                let checked_params = self.check_params(ctx, &params);
                let checked_return_type = self.check_type_annotation(ctx, return_type);

                TypeKind::FnType(CheckedFnType {
                    params: checked_params,
                    return_type: Box::new(checked_return_type),
                })
            }
            TypeAnnotationKind::Struct(items) => {
                let checked_field_types: Vec<CheckedParam> = items
                    .into_iter()
                    .map(|(identifier, ty)| {
                        let checked_type = self.check_type_annotation(ctx, &ty);
                        CheckedParam {
                            identifier: *identifier,
                            ty: checked_type,
                        }
                    })
                    .collect();

                TypeKind::Struct(checked_field_types)
            }
            TypeAnnotationKind::List(item_type) => {
                let checked_item_type = self.check_type_annotation(ctx, item_type);

                TypeKind::List(Box::new(checked_item_type))
            }
            TypeAnnotationKind::Tag(TagAnnotation {
                identifier,
                value_type,
                span,
            }) => {
                let checked_value_type = value_type
                    .as_ref()
                    .map(|v| Box::new(self.check_type_annotation(ctx, v)));

                TypeKind::Tag(CheckedTagType {
                    identifier: *identifier,
                    value_type: checked_value_type,
                    span: *span,
                })
            }
            TypeAnnotationKind::Union(tag_annotations) => {
                let checked_tag_types: Vec<CheckedTagType> = tag_annotations
                    .into_iter()
                    .map(|t| {
                        let checked_type = t
                            .value_type
                            .as_ref()
                            .map(|t| Box::new(self.check_type_annotation(ctx, t)));

                        CheckedTagType {
                            identifier: t.identifier,
                            value_type: checked_type,
                            span: t.span,
                        }
                    })
                    .collect();

                TypeKind::Union(checked_tag_types)
            }
        };

        Type {
            kind,
            span: annotation.span,
        }
    }
}
