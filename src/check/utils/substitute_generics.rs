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
                        self.errors.push(SemanticError {
                            kind: SemanticErrorKind::CouldNotSubstituteGenericParam {
                                generic_param: gp.clone(),
                                with_type: to_substitute,
                            },
                            span: gp.identifier.span,
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
                span: _,
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

                CheckedTypeKind::FnType {
                    params: substituted_params,
                    return_type: Box::new(substituted_return_type),
                    generic_params: vec![],
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

                CheckedTypeKind::StructDecl(CheckedStructDecl {
                    properties: substituted_props,
                    documentation: decl.documentation.clone(),
                    identifier: decl.identifier, // maybe we should rename this?
                    generic_params: vec![],
                })
            }
            CheckedTypeKind::TypeAliasDecl(decl) => {
                let substituted_value = self.substitute_generics(&decl.value, substitutions);

                CheckedTypeKind::TypeAliasDecl(CheckedTypeAliasDecl {
                    value: Box::new(substituted_value),
                    documentation: decl.documentation.clone(),
                    identifier: decl.identifier, // maybe we should rename this?
                    generic_params: vec![],
                })
            }
            CheckedTypeKind::Array { item_type, size } => CheckedTypeKind::Array {
                item_type: Box::new(self.substitute_generics(item_type, substitutions)),
                size: *size,
            },
            CheckedTypeKind::Union(items) => {
                let substituted_items = items.iter().map(|t| self.substitute_generics(t, substitutions));

                // Re-apply union_of logic to simplify the result
                union_of(substituted_items)
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
