use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_type::{TypeAnnotation, TypeAnnotationKind},
        checked::{
            checked_declaration::{CheckedParam, CheckedStructDecl, CheckedTypeAliasDecl},
            checked_type::CheckedType,
        },
        Span,
    },
    check::{
        scope::{Scope, ScopeKind, SymbolEntry},
        SemanticChecker, SemanticError, SemanticErrorKind,
    },
    tokenize::NumberKind,
};

impl<'a> SemanticChecker<'a> {
    pub fn check_type(
        &mut self,
        annotation: &TypeAnnotation,
        scope: Rc<RefCell<Scope>>,
    ) -> CheckedType {
        match &annotation.kind {
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
                let checked_left = self.check_type(&left, scope.clone());
                let checked_args: Vec<(Span, CheckedType)> = args
                    .into_iter()
                    .map(|arg| (arg.span, self.check_type(&arg, scope.clone())))
                    .collect();

                match &checked_left {
                    t @ CheckedType::FnType { generic_params, .. }
                    | t @ CheckedType::StructDecl(CheckedStructDecl { generic_params, .. })
                    | t @ CheckedType::TypeAliasDecl(CheckedTypeAliasDecl {
                        generic_params, ..
                    }) => {
                        let substitutions =
                            self.build_substitutions(generic_params, checked_args, annotation.span);
                        let substituted = self.substitute_generics(t, &substitutions);

                        substituted
                    }
                    _ => {
                        self.errors.push(SemanticError {
                            kind: SemanticErrorKind::CannotApplyTypeArguments {
                                to: checked_left.clone(),
                            },
                            span: left.span,
                        });

                        CheckedType::Unknown
                    }
                }
            }
            TypeAnnotationKind::Identifier(id) => scope
                .borrow()
                .lookup(id.name)
                .map(|entry| match entry {
                    SymbolEntry::StructDecl(decl) => CheckedType::StructDecl(decl),
                    SymbolEntry::EnumDecl(decl) => CheckedType::EnumDecl(decl),
                    SymbolEntry::TypeAliasDecl(decl) => CheckedType::TypeAliasDecl(decl),
                    SymbolEntry::GenericParam(generic_param) => {
                        CheckedType::GenericParam(generic_param)
                    }
                    SymbolEntry::VarDecl(_) => {
                        self.errors.push(SemanticError {
                            kind: SemanticErrorKind::CannotUseVariableDeclarationAsType,
                            span: annotation.span,
                        });
                        CheckedType::Unknown
                    }
                })
                .unwrap_or_else(|| {
                    self.errors.push(SemanticError {
                        kind: SemanticErrorKind::UndeclaredType(*id),
                        span: annotation.span,
                    });
                    CheckedType::Unknown
                }),

            TypeAnnotationKind::FnType {
                params,
                return_type,
                generic_params,
            } => {
                let fn_type_scope = scope.borrow().child(ScopeKind::FnType);

                let checked_generic_params =
                    self.check_generic_params(&generic_params, fn_type_scope.clone());

                let checked_params = params
                    .into_iter()
                    .map(|p| CheckedParam {
                        constraint: self.check_type(&p.constraint, fn_type_scope.clone()),
                        identifier: p.identifier,
                    })
                    .collect();

                CheckedType::FnType {
                    params: checked_params,
                    return_type: Box::new(self.check_type(&return_type, fn_type_scope.clone())),
                    generic_params: checked_generic_params,
                }
            }
            TypeAnnotationKind::Union(items) => CheckedType::Union(
                items
                    .iter()
                    .map(|i| self.check_type(&i, scope.clone()))
                    .collect(),
            ),
            TypeAnnotationKind::Array {
                item_type: left,
                size,
            } => {
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
                        let item_type = self.check_type(&left, scope.clone());
                        CheckedType::Array {
                            item_type: Box::new(item_type),
                            size: valid_size,
                        }
                    }
                    None => {
                        self.errors.push(SemanticError {
                            kind: SemanticErrorKind::InvalidArraySizeValue(*size),
                            span: annotation.span,
                        });
                        let _ = self.check_type(&left, scope.clone()); // Process for errors, ignore result
                        CheckedType::Unknown
                    }
                }
            }
        }
    }
}
