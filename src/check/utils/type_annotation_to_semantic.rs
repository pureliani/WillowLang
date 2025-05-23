use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_type::{TypeAnnotation, TypeAnnotationKind},
        checked::{
            checked_declaration::{
                CheckedGenericStructDecl, CheckedGenericTypeAliasDecl, CheckedParam,
            },
            checked_type::CheckedType,
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
    match &arg.kind {
        TypeAnnotationKind::Void => CheckedType::Void,
        TypeAnnotationKind::Null => CheckedType::Null,
        TypeAnnotationKind::Bool => CheckedType::Bool,
        TypeAnnotationKind::U8 => CheckedType::U8,
        TypeAnnotationKind::U16 => CheckedType::U16,
        TypeAnnotationKind::U32 => CheckedType::U32,
        TypeAnnotationKind::U64 => CheckedType::U64,
        TypeAnnotationKind::USize => CheckedType::USize,
        TypeAnnotationKind::ISize => CheckedType::ISize,
        TypeAnnotationKind::I8 => CheckedType::I8,
        TypeAnnotationKind::I16 => CheckedType::I16,
        TypeAnnotationKind::I32 => CheckedType::I32,
        TypeAnnotationKind::I64 => CheckedType::I64,
        TypeAnnotationKind::F32 => CheckedType::F32,
        TypeAnnotationKind::F64 => CheckedType::F64,
        TypeAnnotationKind::Char => CheckedType::Char,
        TypeAnnotationKind::GenericApply { left, args } => {
            let checked_target = check_type(&left, errors, scope.clone());
            let checked_args: Vec<CheckedType> = args
                .into_iter()
                .map(|arg| check_type(&arg, errors, scope.clone()))
                .collect();

            match checked_target {
                CheckedType::GenericFnType {
                    params,
                    return_type,
                    generic_params,
                } => {
                    todo!("Return specialized type")
                }
                CheckedType::GenericStructDecl(CheckedGenericStructDecl {
                    identifier,
                    generic_params,
                    documentation,
                    properties,
                }) => {
                    todo!("Return specialized type")
                }
                CheckedType::GenericTypeAliasDecl(CheckedGenericTypeAliasDecl {
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
                SymbolEntry::GenericStructDecl(decl) => CheckedType::GenericStructDecl(decl),
                SymbolEntry::StructDecl(decl) => CheckedType::StructDecl(decl),
                SymbolEntry::EnumDecl(decl) => CheckedType::EnumDecl(decl),
                SymbolEntry::GenericTypeAliasDecl(decl) => CheckedType::GenericTypeAliasDecl(decl),
                SymbolEntry::TypeAliasDecl(decl) => CheckedType::TypeAliasDecl(decl),
                SymbolEntry::GenericParam(generic_param) => {
                    CheckedType::GenericParam(generic_param)
                }
                SymbolEntry::VarDecl(_) => {
                    errors.push(SemanticError {
                        kind: SemanticErrorKind::CannotUseVariableDeclarationAsType,
                        span: arg.span,
                    });
                    CheckedType::Unknown
                }
            })
            .unwrap_or_else(|| {
                errors.push(SemanticError {
                    kind: SemanticErrorKind::UndeclaredType(id.name.clone()),
                    span: arg.span,
                });
                CheckedType::Unknown
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

            CheckedType::GenericFnType {
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

            CheckedType::FnType {
                params: checked_params,
                return_type: Box::new(check_type(&return_type, errors, fn_type_scope.clone())),
            }
        }
        TypeAnnotationKind::Union(items) => CheckedType::Union(
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
                    CheckedType::Array {
                        item_type: Box::new(item_type),
                        size: valid_size,
                    }
                }
                None => {
                    errors.push(SemanticError {
                        kind: SemanticErrorKind::InvalidArraySizeValue(*size),
                        span: arg.span,
                    });
                    let _ = check_type(&left, errors, scope.clone()); // Process for errors, ignore result
                    CheckedType::Unknown
                }
            }
        }
    }
}
