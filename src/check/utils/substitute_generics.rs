use std::collections::HashMap;

use crate::{
    ast::checked::{
        checked_declaration::{CheckedParam, CheckedStructDecl, CheckedTypeAliasDecl},
        checked_type::CheckedType,
    },
    check::{SemanticChecker, SemanticError, SemanticErrorKind},
    compile::string_interner::InternerId,
};

use super::union_of::union_of;

pub type GenericSubstitutionMap = HashMap<InternerId, CheckedType>;

impl<'a> SemanticChecker<'a> {
    pub fn substitute_generics(
        &mut self,
        ty: &CheckedType,
        substitutions: &GenericSubstitutionMap,
    ) -> CheckedType {
        match ty {
            CheckedType::GenericParam(gp) => {
                let to_substitute = substitutions
                    .get(&gp.identifier.name)
                    .cloned()
                    .unwrap_or_else(|| {
                        self.errors.push(SemanticError {
                            kind: SemanticErrorKind::UnresolvedGenericParam(gp.identifier),
                            span: gp.identifier.span,
                        });

                        CheckedType::Unknown
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
            CheckedType::FnType {
                params,
                return_type,
                generic_params: _,
            } => {
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

                CheckedType::FnType {
                    params: substituted_params,
                    return_type: Box::new(substituted_return_type),
                    generic_params: vec![],
                }
            }
            CheckedType::StructDecl(decl) => {
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

                CheckedType::StructDecl(CheckedStructDecl {
                    properties: substituted_props,
                    documentation: decl.documentation.clone(),
                    identifier: decl.identifier, // maybe we should rename this?
                    generic_params: vec![],
                })
            }
            CheckedType::TypeAliasDecl(decl) => {
                let substituted_value = self.substitute_generics(&decl.value, substitutions);

                CheckedType::TypeAliasDecl(CheckedTypeAliasDecl {
                    value: Box::new(substituted_value),
                    documentation: decl.documentation.clone(),
                    identifier: decl.identifier, // maybe we should rename this?
                    generic_params: vec![],
                })
            }
            CheckedType::Array { item_type, size } => CheckedType::Array {
                item_type: Box::new(self.substitute_generics(item_type, substitutions)),
                size: *size,
            },
            CheckedType::Union(items) => {
                let substituted_items = items
                    .iter()
                    .map(|t| self.substitute_generics(t, substitutions));

                // Re-apply union_of logic to simplify the result
                union_of(substituted_items)
            }
            CheckedType::I8
            | CheckedType::I16
            | CheckedType::I32
            | CheckedType::I64
            | CheckedType::ISize
            | CheckedType::U8
            | CheckedType::U16
            | CheckedType::U32
            | CheckedType::U64
            | CheckedType::USize
            | CheckedType::F32
            | CheckedType::F64
            | CheckedType::Bool
            | CheckedType::Char
            | CheckedType::Void
            | CheckedType::Null
            | CheckedType::Unknown
            | CheckedType::EnumDecl(_) => ty.clone(),
        }
    }
}
