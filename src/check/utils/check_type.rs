use crate::{
    ast::{
        base::{
            base_declaration::{GenericParam, Param},
            base_type::{TypeAnnotation, TypeAnnotationKind},
        },
        checked::{
            checked_declaration::{CheckedFnType, CheckedGenericParam, CheckedParam},
            checked_type::{CheckedType, CheckedTypeKind},
        },
    },
    check::{
        utils::{
            scope::{ScopeKind, SymbolEntry},
            substitute_generics::GenericSubstitutionMap,
        },
        SemanticChecker, SemanticError,
    },
    tokenize::NumberKind,
};

impl<'a> SemanticChecker<'a> {
    pub fn check_has_type_arguments(&mut self, target: CheckedType) -> CheckedType {
        let has_type_args = match &target.kind {
            CheckedTypeKind::StructDecl(decl) => decl.borrow().generic_params.is_empty(),
            CheckedTypeKind::TypeAliasDecl(decl) => decl.borrow().generic_params.is_empty(),
            CheckedTypeKind::FnType(_) => true,
            _ => true,
        };

        if !has_type_args {
            self.errors.push(SemanticError::ExpectedTypeArguments { span: target.span });
            CheckedType {
                kind: CheckedTypeKind::Unknown,
                span: target.span,
            }
        } else {
            target
        }
    }

    pub fn check_generic_params(&mut self, generic_params: &[GenericParam]) -> Vec<CheckedGenericParam> {
        generic_params
            .into_iter()
            .map(|gp| {
                let checked_constraint = gp.constraint.as_ref().map(|constraint| {
                    let checked_constraint = self.check_type_annotation_recursive(constraint);
                    let result = self.check_has_type_arguments(checked_constraint);
                    Box::new(result)
                });

                let checked_gp = CheckedGenericParam {
                    constraint: checked_constraint,
                    identifier: gp.identifier,
                };

                self.scope_insert(gp.identifier, SymbolEntry::GenericParam(checked_gp.clone()));

                checked_gp
            })
            .collect()
    }

    pub fn check_params(&mut self, params: &Vec<Param>) -> Vec<CheckedParam> {
        params
            .into_iter()
            .map(|p| {
                let definition_id = self.get_definition_id();
                let checked_constraint = self.check_type_annotation_recursive(&p.constraint);
                let result = self.check_has_type_arguments(checked_constraint);
                CheckedParam {
                    id: definition_id,
                    constraint: result,
                    identifier: p.identifier,
                }
            })
            .collect()
    }

    pub fn check_type_annotation_recursive(&mut self, annotation: &TypeAnnotation) -> CheckedType {
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
                let checked_left = self.check_type_annotation_recursive(&left);
                let checked_args: Vec<CheckedType> = args
                    .into_iter()
                    .map(|arg_annotation| {
                        let checked_arg = self.check_type_annotation_recursive(&arg_annotation);
                        let result = self.check_has_type_arguments(checked_arg);
                        result
                    })
                    .collect();

                let mut substitute = |generic_params: &Vec<CheckedGenericParam>, type_args: Vec<CheckedType>| {
                    if generic_params.len() != type_args.len() {
                        self.errors.push(SemanticError::GenericArgumentCountMismatch {
                            expected: generic_params.len(),
                            received: type_args.len(),
                            span: annotation.span,
                        });

                        CheckedTypeKind::Unknown
                    } else {
                        let mut substitutions = GenericSubstitutionMap::new();
                        for (gp_decl, type_arg) in generic_params.iter().zip(type_args.into_iter()) {
                            substitutions.insert(gp_decl.identifier.name, type_arg);
                        }

                        self.substitute_generics(&checked_left, &substitutions).kind
                    }
                };

                match &checked_left.kind {
                    CheckedTypeKind::FnType(decl) => substitute(&decl.generic_params, checked_args),
                    CheckedTypeKind::StructDecl(decl) => substitute(&decl.borrow().generic_params, checked_args),
                    CheckedTypeKind::TypeAliasDecl(decl) => substitute(&decl.borrow().generic_params, checked_args),
                    _ => {
                        self.errors.push(SemanticError::CannotApplyTypeArguments {
                            to: checked_left.clone(),
                        });

                        CheckedTypeKind::Unknown
                    }
                }
            }
            TypeAnnotationKind::Identifier(id) => self
                .scope_lookup(id.name)
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
                self.enter_scope(ScopeKind::FnType);
                let checked_generic_params = self.check_generic_params(&generic_params);
                let checked_params = self.check_params(&params);
                let partially_checked_return_type = self.check_type_annotation_recursive(return_type);
                let checked_return_type = self.check_has_type_arguments(partially_checked_return_type);
                self.exit_scope();

                CheckedTypeKind::FnType(CheckedFnType {
                    params: checked_params,
                    return_type: Box::new(checked_return_type),
                    generic_params: checked_generic_params,
                    span: annotation.span,
                    applied_type_args: vec![],
                })
            }
            TypeAnnotationKind::Union(items) => CheckedTypeKind::Union(
                items
                    .iter()
                    .map(|i| {
                        let checked_item_type = self.check_type_annotation_recursive(&i);
                        let result_item_type = self.check_has_type_arguments(checked_item_type);
                        result_item_type
                    })
                    .collect(),
            ),
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
                        let item_type = self.check_type_annotation_recursive(&left);
                        let result_item_type = self.check_has_type_arguments(item_type);
                        CheckedTypeKind::Array {
                            item_type: Box::new(result_item_type),
                            size: valid_size,
                        }
                    }
                    None => {
                        self.errors.push(SemanticError::InvalidArraySizeValue {
                            span: annotation.span,
                            value: *size,
                        });

                        // Process for errors, ignore result
                        let result = self.check_type_annotation_recursive(&left);
                        let _ = self.check_has_type_arguments(result);
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

    pub fn check_type_annotation(&mut self, annotation: &TypeAnnotation) -> CheckedType {
        let checked_type = self.check_type_annotation_recursive(annotation);
        let result = self.check_has_type_arguments(checked_type);
        result
    }
}
