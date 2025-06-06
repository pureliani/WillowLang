use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::{base_expression::Expr, base_type::TypeAnnotation},
        checked::{
            checked_declaration::{CheckedFnType, CheckedGenericParam, CheckedStructDecl},
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
        Span,
    },
    check::{scope::Scope, utils::substitute_generics::GenericSubstitutionMap, SemanticChecker, SemanticError},
};

impl<'a> SemanticChecker<'a> {
    pub fn build_substitutions(
        &mut self,
        generic_params: &Vec<CheckedGenericParam>,
        type_args: Vec<CheckedType>,
        span: Span,
    ) -> GenericSubstitutionMap {
        if generic_params.len() != type_args.len() {
            self.errors.push(SemanticError::GenericArgumentCountMismatch {
                expected: generic_params.len(),
                received: type_args.len(),
                span,
            });
        } else {
            generic_params.iter().zip(type_args.iter()).for_each(|(gp, ta)| {
                if let Some(constraint) = &gp.constraint {
                    if !self.check_is_assignable(ta, constraint) {
                        self.errors.push(SemanticError::TypeMismatch {
                            expected: *constraint.clone(),
                            received: ta.clone(),
                        });
                    }
                }
            });
        };

        let substitutions: GenericSubstitutionMap = generic_params
            .into_iter()
            .map(|gp| gp.identifier.name)
            .zip(type_args.into_iter())
            .collect();

        substitutions
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

        let (type_kind, substitutions) = match &checked_left.ty.kind {
            CheckedTypeKind::FnType(CheckedFnType { generic_params, .. }) => {
                let substitutions = self.build_substitutions(generic_params, type_args, span);
                let substituted = self.substitute_generics(&checked_left.ty, &substitutions);

                (substituted, substitutions)
            }
            CheckedTypeKind::StructDecl(CheckedStructDecl { generic_params, .. }) => {
                let substitutions = self.build_substitutions(generic_params, type_args, span);
                let substituted = self.substitute_generics(&checked_left.ty, &substitutions);

                (substituted, substitutions)
            }
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
