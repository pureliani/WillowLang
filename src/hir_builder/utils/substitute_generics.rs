use std::collections::HashMap;

use crate::{
    ast::Span,
    compile::string_interner::InternerId,
    hir_builder::{
        types::{
            checked_declaration::{
                CheckedEnumDecl, CheckedEnumVariant, CheckedFnType, CheckedGenericParam, CheckedParam, CheckedTypeAliasDecl,
            },
            checked_type::{Type, TypeKind},
        },
        HIRBuilder, SemanticError,
    },
};

pub type GenericSubstitutionMap = HashMap<InternerId, Type>;

impl<'a> HIRBuilder<'a> {
    pub fn resolve_applied_type_args(
        &mut self,
        generic_params: &[CheckedGenericParam],
        substitutions: &GenericSubstitutionMap,
        span: Span,
    ) -> Vec<Type> {
        generic_params
            .iter()
            .map(|gp| {
                substitutions.get(&gp.identifier.name).cloned().unwrap_or_else(|| {
                    self.errors.push(SemanticError::UnresolvedGenericParam {
                        param: gp.identifier,
                        span,
                    });
                    Type {
                        kind: TypeKind::Unknown,
                        span,
                    }
                })
            })
            .collect()
    }

    pub fn substitute_generics(&mut self, ty: &Type, substitutions: &GenericSubstitutionMap) -> Type {
        match &ty.kind {
            TypeKind::GenericParam(gp) => {
                let to_substitute = substitutions.get(&gp.identifier.name).cloned().unwrap_or_else(|| {
                    self.errors.push(SemanticError::UnresolvedGenericParam {
                        param: gp.identifier,
                        span: ty.span,
                    });

                    Type {
                        kind: TypeKind::Unknown,
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

                        return Type {
                            kind: TypeKind::Unknown,
                            span: ty.span,
                        };
                    }
                }

                to_substitute
            }
            TypeKind::FnType(CheckedFnType {
                params,
                return_type,
                generic_params,
                span,
                applied_type_args: _,
                id,
            }) => {
                let applied_type_args: Vec<Type> = self.resolve_applied_type_args(&generic_params, substitutions, ty.span);

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

                Type {
                    kind: TypeKind::FnType(CheckedFnType {
                        id: *id,
                        params: substituted_params,
                        return_type: Box::new(substituted_return_type),
                        generic_params: vec![],
                        applied_type_args,
                        span: *span,
                    }),
                    span: ty.span,
                }
            }
            TypeKind::Struct(fields) => {
                let substituted_fields = fields
                    .iter()
                    .map(|f| CheckedParam {
                        identifier: f.identifier,
                        constraint: self.substitute_generics(&f.constraint, substitutions),
                    })
                    .collect();

                Type {
                    kind: TypeKind::Struct(substituted_fields),
                    span: ty.span,
                }
            }
            TypeKind::TypeAliasDecl(decl) => {
                let applied_type_args: Vec<Type> = self.resolve_applied_type_args(&decl.generic_params, substitutions, ty.span);

                let substituted_value = self.substitute_generics(&decl.value, substitutions);

                Type {
                    kind: TypeKind::TypeAliasDecl(CheckedTypeAliasDecl {
                        value: Box::new(substituted_value),
                        documentation: decl.documentation.clone(),
                        identifier: decl.identifier, // maybe we should rename this?
                        generic_params: vec![],
                        span: decl.span,
                        applied_type_args,
                        module_id: decl.module_id,
                    }),
                    span: ty.span,
                }
            }
            TypeKind::Array { item_type, size } => Type {
                kind: TypeKind::Array {
                    item_type: Box::new(self.substitute_generics(item_type, substitutions)),
                    size: *size,
                },
                span: ty.span,
            },
            TypeKind::Pointer(item) => {
                let substituted = self.substitute_generics(item, substitutions);

                Type {
                    kind: TypeKind::Pointer(Box::new(substituted)),
                    span: ty.span,
                }
            }
            TypeKind::Enum(decl) => {
                let applied_type_args: Vec<Type> = self.resolve_applied_type_args(&decl.generic_params, substitutions, ty.span);

                let substituted_variants: Vec<CheckedEnumVariant> = decl
                    .variants
                    .iter()
                    .map(|v| CheckedEnumVariant {
                        identifier: v.identifier,
                        payload_type: v
                            .payload_type
                            .as_ref()
                            .map(|payload_t| self.substitute_generics(payload_t, substitutions)),
                    })
                    .collect();

                Type {
                    kind: TypeKind::Enum(CheckedEnumDecl {
                        identifier: decl.identifier,
                        module_id: decl.module_id,
                        documentation: decl.documentation.clone(),
                        variants: substituted_variants,
                        generic_params: vec![],
                        applied_type_args,
                        span: decl.span,
                    }),
                    span: ty.span,
                }
            }
            TypeKind::EnumVariant { parent_enum, variant } => {
                let substituted_payload = variant
                    .payload_type
                    .as_ref()
                    .map(|pt| self.substitute_generics(pt, substitutions));

                let variant = Box::new(CheckedEnumVariant {
                    payload_type: substituted_payload,
                    identifier: variant.identifier,
                });

                Type {
                    kind: TypeKind::EnumVariant {
                        parent_enum: parent_enum.clone(),
                        variant,
                    },
                    span: ty.span,
                }
            }
            TypeKind::I8
            | TypeKind::I16
            | TypeKind::I32
            | TypeKind::I64
            | TypeKind::ISize
            | TypeKind::U8
            | TypeKind::U16
            | TypeKind::U32
            | TypeKind::U64
            | TypeKind::USize
            | TypeKind::F32
            | TypeKind::F64
            | TypeKind::Bool
            | TypeKind::Char
            | TypeKind::Void
            | TypeKind::Unknown => ty.clone(),
        }
    }
}
