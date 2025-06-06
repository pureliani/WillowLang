use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_type::{TypeAnnotation, TypeAnnotationKind},
        checked::{
            checked_declaration::{CheckedFnType, CheckedParam, CheckedStructDecl, CheckedTypeAliasDecl},
            checked_type::{CheckedType, CheckedTypeKind},
        },
    },
    check::{
        scope::{Scope, ScopeKind, SymbolEntry},
        SemanticChecker, SemanticError,
    },
    tokenize::NumberKind,
};

impl<'a> SemanticChecker<'a> {
    pub fn check_type(&mut self, annotation: &TypeAnnotation, scope: Rc<RefCell<Scope>>) -> CheckedType {
        let kind = match &annotation.kind {
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
                let checked_left = self.check_type(&left, scope.clone());
                let checked_args: Vec<CheckedType> = args.into_iter().map(|arg| self.check_type(&arg, scope.clone())).collect();

                match &checked_left.kind {
                    CheckedTypeKind::FnType(CheckedFnType { generic_params, .. })
                    | CheckedTypeKind::StructDecl(CheckedStructDecl { generic_params, .. })
                    | CheckedTypeKind::TypeAliasDecl(CheckedTypeAliasDecl { generic_params, .. }) => {
                        let substitutions = self.build_substitutions(generic_params, checked_args, annotation.span);
                        let substituted = self.substitute_generics(&checked_left, &substitutions);

                        substituted.kind
                    }
                    _ => {
                        self.errors.push(SemanticError::CannotApplyTypeArguments {
                            to: checked_left.clone(),
                        });

                        CheckedTypeKind::Unknown
                    }
                }
            }
            TypeAnnotationKind::Identifier(id) => scope
                .borrow()
                .lookup(id.name)
                .map(|entry| match entry {
                    SymbolEntry::StructDecl(decl) => CheckedTypeKind::StructDecl(decl),
                    SymbolEntry::EnumDecl(decl) => CheckedTypeKind::EnumDecl(decl),
                    SymbolEntry::TypeAliasDecl(decl) => CheckedTypeKind::TypeAliasDecl(decl),
                    SymbolEntry::GenericParam(decl) => CheckedTypeKind::GenericParam(decl),
                    SymbolEntry::VarDecl(_) => {
                        self.errors
                            .push(SemanticError::CannotUseVariableDeclarationAsType { span: annotation.span });

                        CheckedTypeKind::Unknown
                    }
                })
                .unwrap_or_else(|| {
                    self.errors.push(SemanticError::UndeclaredType { id: *id });
                    CheckedTypeKind::Unknown
                }),

            TypeAnnotationKind::FnType {
                params,
                return_type,
                generic_params,
            } => {
                let fn_type_scope = scope.borrow().child(ScopeKind::FnType);

                let checked_generic_params = self.check_generic_params(&generic_params, fn_type_scope.clone());

                let checked_params = params
                    .into_iter()
                    .map(|p| CheckedParam {
                        constraint: self.check_type(&p.constraint, fn_type_scope.clone()),
                        identifier: p.identifier,
                    })
                    .collect();

                CheckedTypeKind::FnType(CheckedFnType {
                    params: checked_params,
                    return_type: Box::new(self.check_type(&return_type, fn_type_scope.clone())),
                    generic_params: checked_generic_params,
                    span: annotation.span,
                })
            }
            TypeAnnotationKind::Union(items) => {
                CheckedTypeKind::Union(items.iter().map(|i| self.check_type(&i, scope.clone())).collect())
            }
            TypeAnnotationKind::Array { item_type: left, size } => {
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
                        CheckedTypeKind::Array {
                            item_type: Box::new(item_type),
                            size: valid_size,
                        }
                    }
                    None => {
                        self.errors.push(SemanticError::InvalidArraySizeValue {
                            span: annotation.span,
                            value: *size,
                        });
                        let _ = self.check_type(&left, scope.clone()); // Process for errors, ignore result
                        CheckedTypeKind::Unknown
                    }
                }
            }
        };

        CheckedType {
            kind,
            span: annotation.span,
        }
    }
}
