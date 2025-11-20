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
            checked_type::{CheckedStruct, Type},
        },
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn check_params(
        &mut self,
        ctx: &mut HIRContext,
        params: &[Param],
    ) -> Vec<CheckedParam> {
        params
            .iter()
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
    ) -> Result<Type, SemanticError> {
        ctx.module_builder
            .scope_lookup(id.name)
            .map(|entry| match entry {
                CheckedDeclaration::TypeAlias(decl) => {
                    Ok(decl.read().unwrap().value.as_ref().clone())
                }
                CheckedDeclaration::Function(_) => Err(SemanticError {
                    kind: SemanticErrorKind::CannotUseFunctionDeclarationAsType,
                    span,
                }),
                CheckedDeclaration::Var(_)
                | CheckedDeclaration::UninitializedVar { .. } => Err(SemanticError {
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

    pub fn check_tag_annotation(
        &mut self,
        ctx: &mut HIRContext,
        TagAnnotation {
            identifier,
            value_type,
            span,
        }: &TagAnnotation,
    ) -> Type {
        let checked_value_type = value_type
            .as_ref()
            .map(|v| self.check_type_annotation(ctx, v));

        // TODO: figure out what to do with the semantic type
        let semantic_type = CheckedTagType {
            identifier: *identifier,
            value_type: checked_value_type.clone().map(Box::new),
            span: *span,
        };

        Type::Struct(CheckedStruct::tag(&ctx.program_builder, checked_value_type))
    }

    pub fn check_type_annotation(
        &mut self,
        ctx: &mut HIRContext,
        annotation: &TypeAnnotation,
    ) -> Type {
        let kind = match &annotation.kind {
            TypeAnnotationKind::Void => Type::Void,
            TypeAnnotationKind::Bool => Type::Bool,
            TypeAnnotationKind::U8 => Type::U8,
            TypeAnnotationKind::U16 => Type::U16,
            TypeAnnotationKind::U32 => Type::U32,
            TypeAnnotationKind::U64 => Type::U64,
            TypeAnnotationKind::USize => Type::USize,
            TypeAnnotationKind::ISize => Type::ISize,
            TypeAnnotationKind::I8 => Type::I8,
            TypeAnnotationKind::I16 => Type::I16,
            TypeAnnotationKind::I32 => Type::I32,
            TypeAnnotationKind::I64 => Type::I64,
            TypeAnnotationKind::F32 => Type::F32,
            TypeAnnotationKind::F64 => Type::F64,
            TypeAnnotationKind::String => {
                Type::Struct(CheckedStruct::string(&ctx.program_builder))
            }
            TypeAnnotationKind::Identifier(id) => {
                match self.check_type_identifier_annotation(ctx, *id, annotation.span) {
                    Ok(resolved_type) => {
                        return resolved_type;
                    }
                    Err(error) => {
                        ctx.program_builder.errors.push(error);
                        Type::Unknown
                    }
                }
            }
            TypeAnnotationKind::FnType {
                params,
                return_type,
            } => {
                let checked_params = self.check_params(ctx, params);
                let checked_return_type = self.check_type_annotation(ctx, return_type);

                // TODO: figure out
                let semantic_type = CheckedFnType {
                    params: checked_params,
                    return_type: Box::new(checked_return_type),
                };

                Type::Struct(CheckedStruct::closure(&ctx.program_builder))
            }
            TypeAnnotationKind::Struct(items) => {
                let mut checked_field_types: Vec<CheckedParam> = items
                    .iter()
                    .map(|(identifier, ty)| {
                        let checked_type = self.check_type_annotation(ctx, ty);
                        CheckedParam {
                            identifier: *identifier,
                            ty: checked_type,
                        }
                    })
                    .collect();

                Type::Struct(CheckedStruct::user_defined(
                    &ctx.program_builder,
                    &mut checked_field_types,
                ))
            }
            TypeAnnotationKind::List(item_type) => {
                let checked_item_type = self.check_type_annotation(ctx, item_type);

                Type::Struct(CheckedStruct::list(&ctx.program_builder, checked_item_type))
            }
            TypeAnnotationKind::Tag(t) => self.check_tag_annotation(ctx, t),
            TypeAnnotationKind::Union(tag_annotations) => {
                let checked_tag_types: Vec<Type> = tag_annotations
                    .iter()
                    .map(|t| self.check_tag_annotation(ctx, t))
                    .collect();

                Type::Struct(CheckedStruct::union(
                    &ctx.program_builder,
                    &checked_tag_types,
                ))
            }
        };

        kind
    }
}
