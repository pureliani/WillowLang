use crate::{
    ast::{
        decl::Param,
        type_annotation::{TypeAnnotation, TypeAnnotationKind},
    },
    hir_builder::{
        errors::SemanticError,
        types::{
            checked_declaration::{CheckedFnType, CheckedParam, CheckedTag},
            checked_type::{Type, TypeKind},
        },
        utils::scope::{ScopeKind, SymbolEntry},
        HIRBuilder,
    },
};

impl<'a> HIRBuilder<'a> {
    pub fn check_params(&mut self, params: &Vec<Param>) -> Vec<CheckedParam> {
        params
            .into_iter()
            .map(|p| CheckedParam {
                constraint: self.check_type_annotation_recursive(&p.constraint),
                identifier: p.identifier,
            })
            .collect()
    }

    pub fn check_type_annotation_recursive(&mut self, annotation: &TypeAnnotation) -> Type {
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
            TypeAnnotationKind::Struct { fields } => TypeKind::Struct(self.check_params(fields)),
            TypeAnnotationKind::Identifier { identifier: id } => self
                .scope_lookup(id.name)
                .map(|entry| match entry {
                    SymbolEntry::TypeAliasDecl(decl) => TypeKind::TypeAliasDecl(decl),
                    SymbolEntry::VarDecl(_) => {
                        self.errors
                            .push(SemanticError::CannotUseVariableDeclarationAsType { span: annotation.span });

                        TypeKind::Unknown
                    }
                })
                .unwrap_or_else(|| {
                    self.errors.push(SemanticError::UndeclaredType { id: *id });
                    TypeKind::Unknown
                }),
            TypeAnnotationKind::FnType { params, return_type } => {
                self.enter_scope(ScopeKind::FnType);
                let checked_params = self.check_params(&params);
                let checked_return_type = self.check_type_annotation_recursive(return_type);
                self.exit_scope();

                TypeKind::FnType(CheckedFnType {
                    params: checked_params,
                    return_type: Box::new(checked_return_type),
                    span: annotation.span,
                })
            }
            TypeAnnotationKind::List { item_type } => {
                let checked_item_type = self.check_type_annotation_recursive(item_type);
                TypeKind::List(Box::new(checked_item_type))
            }
            TypeAnnotationKind::Tag { identifier, value_type } => {
                let checked_value_type = value_type.as_ref().map(|t| Box::new(self.check_type_annotation_recursive(t)));
                TypeKind::Tag(CheckedTag {
                    identifier: *identifier,
                    value_type: checked_value_type,
                })
            }
        };

        Type {
            kind,
            span: annotation.span,
        }
    }

    pub fn check_type_annotation(&mut self, annotation: &TypeAnnotation) -> Type {
        self.check_type_annotation_recursive(annotation)
    }
}
