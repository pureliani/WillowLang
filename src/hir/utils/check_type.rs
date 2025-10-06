use crate::{
    ast::{
        decl::Param,
        type_annotation::{TypeAnnotation, TypeAnnotationKind},
        IdentifierNode, Span,
    },
    hir::{
        errors::{SemanticError, SemanticErrorKind},
        types::{
            checked_declaration::{CheckedFnType, CheckedParam},
            checked_type::{Type, TypeKind},
        },
        utils::scope::{ScopeKind, SymbolEntry},
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn check_params(&mut self, ctx: &mut HIRContext, params: &Vec<Param>) -> Vec<CheckedParam> {
        params
            .into_iter()
            .map(|p| CheckedParam {
                constraint: self.check_type_annotation(ctx, &p.constraint),
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
                SymbolEntry::TypeAliasDecl(decl) => Ok(TypeKind::TypeAliasDecl(decl)),
                SymbolEntry::EnumDecl(decl) => Ok(TypeKind::Enum(decl)),
                SymbolEntry::VarDecl(_) => Err(SemanticError {
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

    pub fn check_type_annotation(&mut self, ctx: &mut HIRContext, annotation: &TypeAnnotation) -> Type {
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
            TypeAnnotationKind::Identifier(id) => match self.check_type_identifier_annotation(ctx, *id, annotation.span) {
                Ok(t) => t,
                Err(error) => {
                    ctx.module_builder.errors.push(error);
                    TypeKind::Unknown
                }
            },
            TypeAnnotationKind::FnType { params, return_type } => {
                ctx.module_builder.enter_scope(ScopeKind::FnType);
                let checked_params = self.check_params(ctx, &params);
                let checked_return_type = self.check_type_annotation(ctx, return_type);
                ctx.module_builder.exit_scope();

                TypeKind::FnType(CheckedFnType {
                    params: checked_params,
                    return_type: Box::new(checked_return_type),
                    span: annotation.span,
                })
            }
            TypeAnnotationKind::Struct(items) => {
                let checked_field_types: Vec<(IdentifierNode, Type)> = items
                    .into_iter()
                    .map(|(identifier, ty)| {
                        let checked_type = self.check_type_annotation(ctx, &ty);
                        (*identifier, checked_type)
                    })
                    .collect();

                TypeKind::Struct(checked_field_types)
            }
        };

        Type {
            kind,
            span: annotation.span,
        }
    }
}
