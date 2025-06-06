use std::collections::HashMap;

use crate::{
    ast::checked::{
        checked_declaration::{CheckedFnType, CheckedParam, CheckedStructDecl, CheckedTypeAliasDecl},
        checked_type::{CheckedType, CheckedTypeKind},
    },
    check::{SemanticChecker, SemanticError},
    compile::string_interner::InternerId,
};

use super::union_of::union_of;

pub type GenericSubstitutionMap = HashMap<InternerId, CheckedType>;

impl<'a> SemanticChecker<'a> {
    pub fn substitute_generics(&mut self, ty: &CheckedType, substitutions: &GenericSubstitutionMap) -> CheckedType {
        match &ty.kind {
            CheckedTypeKind::GenericParam(gp) => {
                let to_substitute = substitutions.get(&gp.identifier.name).cloned().unwrap_or_else(|| {
                    self.errors
                        .push(SemanticError::UnresolvedGenericParam { param: gp.identifier });

                    CheckedType {
                        kind: CheckedTypeKind::Unknown,
                        span: ty.span,
                    }
                });

                match &gp.constraint {
                    Some(c) if !self.check_is_assignable(&to_substitute, c) => {
                        self.errors.push(SemanticError::CouldNotSubstituteGenericParam {
                            generic_param: gp.clone(),
                            with_type: to_substitute,
                        });

                        *c.clone()
                    }
                    _ => to_substitute,
                }
            }
            CheckedTypeKind::FnType(CheckedFnType {
                params,
                return_type,
                generic_params: _,
                span,
            }) => {
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
                        span: *span,
                    }),
                    span: ty.span,
                }
            }
            CheckedTypeKind::StructDecl(decl) => {
                // Similar to FnType, a struct definition's generic params are local.
                // We substitute types *within* its properties if those types refer
                // to generics from the *outer* substitution context.
                let substituted_props = decl
                    .properties
                    .iter()
                    .map(|p| CheckedParam {
                        identifier: p.identifier,
                        constraint: self.substitute_generics(&p.constraint, substitutions),
                    })
                    .collect();

                CheckedType {
                    kind: CheckedTypeKind::StructDecl(CheckedStructDecl {
                        properties: substituted_props,
                        documentation: decl.documentation.clone(),
                        identifier: decl.identifier, // maybe we should rename this?
                        generic_params: vec![],
                        span: decl.span,
                    }),
                    span: ty.span,
                }
            }
            CheckedTypeKind::TypeAliasDecl(decl) => {
                let substituted_value = self.substitute_generics(&decl.value, substitutions);

                CheckedType {
                    kind: CheckedTypeKind::TypeAliasDecl(CheckedTypeAliasDecl {
                        value: Box::new(substituted_value),
                        documentation: decl.documentation.clone(),
                        identifier: decl.identifier, // maybe we should rename this?
                        generic_params: vec![],
                        span: decl.span,
                    }),
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
