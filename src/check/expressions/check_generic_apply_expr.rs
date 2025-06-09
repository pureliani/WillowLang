use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::{base_expression::Expr, base_type::TypeAnnotation},
        checked::{
            checked_declaration::{CheckedFnType, CheckedGenericParam},
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
        Span,
    },
    check::{scope::Scope, utils::substitute_generics::GenericSubstitutionMap, SemanticChecker, SemanticError},
};

#[derive(Debug, Clone)]
pub enum GenericArgumentSource<'b> {
    Explicit(Vec<CheckedType>),
    Inferred { substitutions: &'b GenericSubstitutionMap },
}

impl<'a> SemanticChecker<'a> {
    pub fn build_substitution_map(
        &mut self,
        declared_generic_params: &Vec<CheckedGenericParam>,
        arg_source: GenericArgumentSource,
        error_span: Span,
    ) -> Option<GenericSubstitutionMap> {
        let is_inferred = matches!(arg_source, GenericArgumentSource::Inferred { .. });
        let mut working_substitutions = GenericSubstitutionMap::new();
        let mut preliminary_ok = true;

        match arg_source {
            GenericArgumentSource::Explicit(type_args) => {
                if declared_generic_params.len() != type_args.len() {
                    self.errors.push(SemanticError::GenericArgumentCountMismatch {
                        expected: declared_generic_params.len(),
                        received: type_args.len(),
                        span: error_span,
                    });

                    preliminary_ok = false;
                } else {
                    for (gp_decl, type_arg) in declared_generic_params.iter().zip(type_args.into_iter()) {
                        working_substitutions.insert(gp_decl.identifier.name, type_arg);
                    }
                }
            }
            GenericArgumentSource::Inferred { substitutions } => {
                working_substitutions = substitutions.clone();
            }
        }

        if !preliminary_ok {
            return None;
        }

        let mut all_constraints_ok = true;
        for gp_decl in declared_generic_params {
            if let Some(substituted_type) = working_substitutions.get(&gp_decl.identifier.name) {
                if let Some(constraint) = &gp_decl.constraint {
                    if !self.check_is_assignable(substituted_type, constraint) {
                        self.errors.push(SemanticError::IncompatibleGenericParamSubstitution {
                            generic_param: gp_decl.clone(),
                            arg_type: substituted_type.clone(),
                            is_inferred,
                        });

                        all_constraints_ok = false;
                    }
                }
            } else {
                self.errors.push(SemanticError::UnresolvedGenericParam {
                    param: gp_decl.identifier,
                    span: gp_decl.identifier.span,
                });

                all_constraints_ok = false;
            }
        }

        if all_constraints_ok {
            Some(working_substitutions)
        } else {
            None
        }
    }
}

impl<'a> SemanticChecker<'a> {
    pub fn check_generic_apply_expr(
        &mut self,
        left: Box<Expr>,
        args: Vec<TypeAnnotation>,
        span: Span,
        scope: Rc<RefCell<Scope>>,
    ) -> CheckedExpr {
        let checked_left = self.check_expr(*left, scope.clone());
        let type_args: Vec<_> = args
            .into_iter()
            .map(|type_arg| self.check_type(&type_arg, scope.clone()))
            .collect();

        let mut substitute = |generic_params: &Vec<CheckedGenericParam>, type_args: Vec<CheckedType>| {
            let substitutions_opt = self.build_substitution_map(generic_params, GenericArgumentSource::Explicit(type_args), span);

            if let Some(substitutions) = substitutions_opt {
                let substituted = self.substitute_generics(&checked_left.ty, &substitutions);

                (substituted, substitutions)
            } else {
                (
                    CheckedType {
                        kind: CheckedTypeKind::Unknown,
                        span,
                    },
                    GenericSubstitutionMap::new(),
                )
            }
        };

        let (type_kind, substitutions) = match &checked_left.ty.kind {
            CheckedTypeKind::FnType(CheckedFnType { generic_params, .. }) => substitute(generic_params, type_args),
            CheckedTypeKind::StructDecl(decl) => substitute(&decl.borrow().generic_params, type_args),
            _ => {
                self.errors.push(SemanticError::CannotApplyTypeArguments {
                    to: checked_left.ty.clone(),
                });

                (
                    CheckedType {
                        kind: CheckedTypeKind::Unknown,
                        span: checked_left.ty.span,
                    },
                    GenericSubstitutionMap::new(),
                )
            }
        };

        CheckedExpr {
            ty: type_kind,
            kind: CheckedExprKind::TypeSpecialization {
                target: Box::new(checked_left),
                substitutions,
            },
        }
    }
}
