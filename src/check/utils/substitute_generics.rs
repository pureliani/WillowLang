use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{
        checked::{
            checked_declaration::{CheckedFnType, CheckedGenericParam, CheckedParam, CheckedStructDecl, CheckedTypeAliasDecl},
            checked_type::{CheckedType, CheckedTypeKind},
        },
        Span,
    },
    check::{SemanticChecker, SemanticError},
    compile::string_interner::InternerId,
};

use super::union_of::union_of;

pub type GenericSubstitutionMap = HashMap<InternerId, CheckedType>;

impl<'a> SemanticChecker<'a> {
    pub fn resolve_applied_type_args(
        &mut self,
        generic_params: &[CheckedGenericParam],
        substitutions: &GenericSubstitutionMap,
        span: Span,
    ) -> Vec<CheckedType> {
        generic_params
            .iter()
            .map(|gp| {
                substitutions.get(&gp.identifier.name).cloned().unwrap_or_else(|| {
                    self.errors.push(SemanticError::UnresolvedGenericParam {
                        param: gp.identifier,
                        span,
                    });
                    CheckedType {
                        kind: CheckedTypeKind::Unknown,
                        span,
                    }
                })
            })
            .collect()
    }

    pub fn substitute_generics(&mut self, ty: &CheckedType, substitutions: &GenericSubstitutionMap) -> CheckedType {
        match &ty.kind {
            CheckedTypeKind::GenericParam(gp) => {
                let to_substitute = substitutions.get(&gp.identifier.name).cloned().unwrap_or_else(|| {
                    self.errors.push(SemanticError::UnresolvedGenericParam {
                        param: gp.identifier,
                        span: ty.span,
                    });

                    CheckedType {
                        kind: CheckedTypeKind::Unknown,
                        span: ty.span,
                    }
                });

                if let Some(c) = &gp.constraint {
                    if !self.check_is_assignable(&to_substitute, c) {
                        self.errors.push(SemanticError::IncompatibleGenericParamSubstitution {
                            generic_param: gp.clone(),
                            arg_type: to_substitute.clone(),
                            is_inferred: true,
                        });

                        return CheckedType {
                            kind: CheckedTypeKind::Unknown,
                            span: ty.span,
                        };
                    }
                }

                to_substitute
            }
            CheckedTypeKind::FnType(CheckedFnType {
                params,
                return_type,
                generic_params,
                span,
                applied_type_args: _,
            }) => {
                let applied_type_args: Vec<CheckedType> = self.resolve_applied_type_args(&generic_params, substitutions, ty.span);

                // IMPORTANT: When substituting within a function type, we DON'T
                // substitute its *own* generic parameters.
                // We only substitute types that came from an outer scope's substitution.
                let substituted_params = params
                    .iter()
                    .map(|p| CheckedParam {
                        identifier: p.identifier,
                        constraint: self.substitute_generics(&p.constraint, substitutions),
                    })
                    .collect();

                let substituted_return_type = self.substitute_generics(return_type, substitutions);

                CheckedType {
                    kind: CheckedTypeKind::FnType(CheckedFnType {
                        params: substituted_params,
                        return_type: Box::new(substituted_return_type),
                        generic_params: vec![],
                        applied_type_args,
                        span: *span,
                    }),
                    span: ty.span,
                }
            }
            CheckedTypeKind::StructDecl(decl) => {
                let decl = decl.borrow();
                let applied_type_args: Vec<CheckedType> =
                    self.resolve_applied_type_args(&decl.generic_params, substitutions, ty.span);

                let substituted_fields = decl
                    .fields
                    .iter()
                    .map(|p| CheckedParam {
                        identifier: p.identifier,
                        constraint: self.substitute_generics(&p.constraint, substitutions),
                    })
                    .collect();

                CheckedType {
                    kind: CheckedTypeKind::StructDecl(Rc::new(RefCell::new(CheckedStructDecl {
                        fields: substituted_fields,
                        documentation: decl.documentation.clone(),
                        identifier: decl.identifier, // maybe we should rename this?
                        generic_params: vec![],
                        span: decl.span,
                        applied_type_args,
                    }))),
                    span: ty.span,
                }
            }
            CheckedTypeKind::TypeAliasDecl(decl) => {
                let decl = decl.borrow();
                let applied_type_args: Vec<CheckedType> =
                    self.resolve_applied_type_args(&decl.generic_params, substitutions, ty.span);

                let substituted_value = self.substitute_generics(&decl.value, substitutions);

                CheckedType {
                    kind: CheckedTypeKind::TypeAliasDecl(Rc::new(RefCell::new(CheckedTypeAliasDecl {
                        value: Box::new(substituted_value),
                        documentation: decl.documentation.clone(),
                        identifier: decl.identifier, // maybe we should rename this?
                        generic_params: vec![],
                        span: decl.span,
                        applied_type_args,
                    }))),
                    span: ty.span,
                }
            }
            CheckedTypeKind::Array { item_type, size } => CheckedType {
                kind: CheckedTypeKind::Array {
                    item_type: Box::new(self.substitute_generics(item_type, substitutions)),
                    size: *size,
                },
                span: ty.span,
            },
            CheckedTypeKind::Union(items) => {
                let substituted_items = items.iter().map(|t| self.substitute_generics(t, substitutions));

                // Re-apply union_of logic to simplify the result
                union_of(substituted_items, ty.span)
            }
            CheckedTypeKind::I8
            | CheckedTypeKind::I16
            | CheckedTypeKind::I32
            | CheckedTypeKind::I64
            | CheckedTypeKind::ISize
            | CheckedTypeKind::U8
            | CheckedTypeKind::U16
            | CheckedTypeKind::U32
            | CheckedTypeKind::U64
            | CheckedTypeKind::USize
            | CheckedTypeKind::F32
            | CheckedTypeKind::F64
            | CheckedTypeKind::Bool
            | CheckedTypeKind::Char
            | CheckedTypeKind::Void
            | CheckedTypeKind::Null
            | CheckedTypeKind::Unknown
            | CheckedTypeKind::EnumDecl(_) => ty.clone(),
        }
    }
}
