use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_type::{TypeAnnotation, TypeAnnotationKind},
        checked::{
            checked_declaration::CheckedParam,
            checked_type::{Type, TypeKind, TypeSpan},
        },
    },
    tokenizer::NumberKind,
};

use super::{
    check_stmt::check_generic_params,
    scope::{Scope, ScopeKind, SymbolEntry},
    SemanticError, SemanticErrorKind,
};

pub fn type_annotation_to_semantic(
    arg: &TypeAnnotation,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> Type {
    let kind = match &arg.kind {
        TypeAnnotationKind::Void => TypeKind::Void,
        TypeAnnotationKind::Null => TypeKind::Null,
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
        TypeAnnotationKind::Char => TypeKind::Char,
        TypeAnnotationKind::GenericApply { left, args } => todo!(),
        TypeAnnotationKind::Identifier(id) => scope
            .borrow()
            .lookup(&id.name)
            .map(|entry| match entry {
                SymbolEntry::StructDecl(s) => TypeKind::Struct(s),
                SymbolEntry::EnumDecl(decl) => TypeKind::Enum(decl),
                SymbolEntry::TypeAliasDecl(decl) => TypeKind::TypeAlias(decl),
                SymbolEntry::GenericParam(generic_param) => TypeKind::GenericParam(generic_param),
                SymbolEntry::VarDecl(_) => {
                    errors.push(SemanticError::new(
                        SemanticErrorKind::CannotUseVariableDeclarationAsType,
                        arg.span,
                    ));
                    TypeKind::Unknown
                }
            })
            .unwrap_or_else(|| {
                errors.push(SemanticError::new(
                    SemanticErrorKind::UndeclaredType(id.name.clone()),
                    arg.span,
                ));
                TypeKind::Unknown
            }),
        TypeAnnotationKind::FnType {
            params,
            return_type,
            generic_params,
        } => {
            let fn_type_scope = scope.borrow().child(ScopeKind::FnType);

            let checked_generic_params =
                check_generic_params(&generic_params, errors, fn_type_scope.clone());

            let checked_params = params
                .into_iter()
                .map(|p| CheckedParam {
                    constraint: type_annotation_to_semantic(
                        &p.constraint,
                        errors,
                        fn_type_scope.clone(),
                    ),
                    identifier: p.identifier.clone(),
                })
                .collect();

            TypeKind::FnType {
                params: checked_params,
                return_type: Box::new(type_annotation_to_semantic(
                    &return_type,
                    errors,
                    fn_type_scope.clone(),
                )),
                generic_params: checked_generic_params,
            }
        }
        TypeAnnotationKind::Union(items) => TypeKind::Union(
            items
                .iter()
                .map(|i| type_annotation_to_semantic(&i, errors, scope.clone()))
                .collect(),
        ),
        TypeAnnotationKind::Array { left, size } => {
            let maybe_size: Option<usize> = match size {
                &NumberKind::USize(v) => Some(v),
                &NumberKind::U64(v) => v.try_into().ok(),
                &NumberKind::U32(v) => v.try_into().ok(),
                &NumberKind::U16(v) => Some(v as usize),
                &NumberKind::U8(v) => Some(v as usize),

                &NumberKind::ISize(v) => v.try_into().ok(),
                &NumberKind::I64(v) => v.try_into().ok(),
                &NumberKind::I32(v) => v.try_into().ok(),
                &NumberKind::I16(v) => v.try_into().ok(),
                &NumberKind::I8(v) => v.try_into().ok(),

                &NumberKind::F32(_) | &NumberKind::F64(_) => None,
            };

            match maybe_size {
                Some(valid_size) => {
                    let item_type = type_annotation_to_semantic(&left, errors, scope.clone());
                    TypeKind::Array {
                        item_type: Box::new(item_type),
                        size: valid_size,
                    }
                }
                None => {
                    errors.push(SemanticError::new(
                        SemanticErrorKind::InvalidArraySizeValue(*size),
                        arg.span,
                    ));
                    let _ = type_annotation_to_semantic(&left, errors, scope.clone()); // Process for errors, ignore result
                    TypeKind::Unknown
                }
            }
        }
        TypeAnnotationKind::Error(_) => TypeKind::Unknown,
    };

    Type {
        kind,
        span: TypeSpan::Annotation(arg.span),
    }
}
