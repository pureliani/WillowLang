use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_type::{TypeAnnotation, TypeAnnotationKind},
        checked::{
            checked_declaration::{CheckedParam, GenericStructDecl, GenericTypeAliasDecl},
            checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
        },
    },
    check::{
        check_stmt::check_generic_params,
        scope::{Scope, ScopeKind, SymbolEntry},
        SemanticError, SemanticErrorKind,
    },
    tokenizer::NumberKind,
};

pub fn check_type(
    arg: &TypeAnnotation,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedType {
    let kind = match &arg.kind {
        TypeAnnotationKind::Void => CheckedTypeKind::Void,
        TypeAnnotationKind::Null => CheckedTypeKind::Null,
        TypeAnnotationKind::Bool => CheckedTypeKind::Bool,
        TypeAnnotationKind::U8 => CheckedTypeKind::U8,
        TypeAnnotationKind::U16 => CheckedTypeKind::U16,
        TypeAnnotationKind::U32 => CheckedTypeKind::U32,
        TypeAnnotationKind::U64 => CheckedTypeKind::U64,
        TypeAnnotationKind::USize => CheckedTypeKind::USize,
        TypeAnnotationKind::ISize => CheckedTypeKind::ISize,
        TypeAnnotationKind::I8 => CheckedTypeKind::I8,
        TypeAnnotationKind::I16 => CheckedTypeKind::I16,
        TypeAnnotationKind::I32 => CheckedTypeKind::I32,
        TypeAnnotationKind::I64 => CheckedTypeKind::I64,
        TypeAnnotationKind::F32 => CheckedTypeKind::F32,
        TypeAnnotationKind::F64 => CheckedTypeKind::F64,
        TypeAnnotationKind::Char => CheckedTypeKind::Char,
        TypeAnnotationKind::GenericApply { left, args } => {
            let checked_target = check_type(&left, errors, scope.clone());
            let checked_args = args
                .into_iter()
                .map(|arg| check_type(&arg, errors, scope.clone()))
                .collect::<Vec<CheckedType>>();

            match checked_target.kind {
                CheckedTypeKind::GenericFnType {
                    params,
                    return_type,
                    generic_params,
                } => {
                    todo!("Return specialized type")
                }
                CheckedTypeKind::GenericStructDecl(GenericStructDecl {
                    identifier,
                    generic_params,
                    documentation,
                    properties,
                }) => {
                    todo!("Return specialized type")
                }
                CheckedTypeKind::GenericTypeAliasDecl(GenericTypeAliasDecl {
                    identifier,
                    generic_params,
                    documentation,
                    value,
                }) => {
                    todo!("Return specialized type")
                }
                _ => {
                    todo!("Push an error when target is non generic type")
                }
            }
        }
        TypeAnnotationKind::Identifier(id) => scope
            .borrow()
            .lookup(&id.name)
            .map(|entry| match entry {
                SymbolEntry::GenericStructDecl(decl) => CheckedTypeKind::GenericStructDecl(decl),
                SymbolEntry::StructDecl(decl) => CheckedTypeKind::StructDecl(decl),
                SymbolEntry::EnumDecl(decl) => CheckedTypeKind::Enum(decl),
                SymbolEntry::GenericTypeAliasDecl(decl) => {
                    CheckedTypeKind::GenericTypeAliasDecl(decl)
                }
                SymbolEntry::TypeAliasDecl(decl) => CheckedTypeKind::TypeAliasDecl(decl),
                SymbolEntry::GenericParam(generic_param) => {
                    CheckedTypeKind::GenericParam(generic_param)
                }
                SymbolEntry::VarDecl(_) => {
                    errors.push(SemanticError::new(
                        SemanticErrorKind::CannotUseVariableDeclarationAsType,
                        arg.span,
                    ));
                    CheckedTypeKind::Unknown
                }
            })
            .unwrap_or_else(|| {
                errors.push(SemanticError::new(
                    SemanticErrorKind::UndeclaredType(id.name.clone()),
                    arg.span,
                ));
                CheckedTypeKind::Unknown
            }),

        TypeAnnotationKind::GenericFnType {
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
                    constraint: check_type(&p.constraint, errors, fn_type_scope.clone()),
                    identifier: p.identifier.clone(),
                })
                .collect();

            CheckedTypeKind::GenericFnType {
                params: checked_params,
                return_type: Box::new(check_type(&return_type, errors, fn_type_scope.clone())),
                generic_params: checked_generic_params,
            }
        }
        TypeAnnotationKind::FnType {
            params,
            return_type,
        } => {
            let fn_type_scope = scope.borrow().child(ScopeKind::FnType);

            let checked_params = params
                .into_iter()
                .map(|p| CheckedParam {
                    constraint: check_type(&p.constraint, errors, fn_type_scope.clone()),
                    identifier: p.identifier.clone(),
                })
                .collect();

            CheckedTypeKind::FnType {
                params: checked_params,
                return_type: Box::new(check_type(&return_type, errors, fn_type_scope.clone())),
            }
        }
        TypeAnnotationKind::Union(items) => CheckedTypeKind::Union(
            items
                .iter()
                .map(|i| check_type(&i, errors, scope.clone()))
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
                    let item_type = check_type(&left, errors, scope.clone());
                    CheckedTypeKind::Array {
                        item_type: Box::new(item_type),
                        size: valid_size,
                    }
                }
                None => {
                    errors.push(SemanticError::new(
                        SemanticErrorKind::InvalidArraySizeValue(*size),
                        arg.span,
                    ));
                    let _ = check_type(&left, errors, scope.clone()); // Process for errors, ignore result
                    CheckedTypeKind::Unknown
                }
            }
        }
        TypeAnnotationKind::Error(_) => CheckedTypeKind::Unknown,
    };

    CheckedType {
        kind,
        span: TypeSpan::Annotation(arg.span),
    }
}
