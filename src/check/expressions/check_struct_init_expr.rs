use std::{cell::RefCell, collections::HashSet, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_declaration::{CheckedParam, CheckedStructDecl},
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
        IdentifierNode, Span,
    },
    check::{
        expressions::check_generic_apply_expr::GenericArgumentSource, scope::Scope,
        utils::substitute_generics::GenericSubstitutionMap, SemanticChecker, SemanticError,
    },
    compile::string_interner::InternerId,
};

impl<'a> SemanticChecker<'a> {
    pub fn check_struct_init_expr(
        &mut self,
        left_expr: Box<Expr>,
        fields: Vec<(IdentifierNode, Expr)>,
        span: Span,
        scope: Rc<RefCell<Scope>>,
    ) -> CheckedExpr {
        let checked_left = self.check_expr(*left_expr, scope.clone());

        let checked_args: Vec<(IdentifierNode, CheckedExpr)> = fields
            .into_iter()
            .map(|(ident, expr)| (ident, self.check_expr(expr, scope.clone())))
            .collect();

        let mut result_struct_type = CheckedType {
            kind: CheckedTypeKind::Unknown,
            span,
        };

        match &checked_left.ty.kind {
            CheckedTypeKind::StructDecl(decl) => {
                let decl = decl.borrow();

                let mut uninitialized_field_tracker: HashSet<InternerId> =
                    decl.fields.iter().map(|p| p.identifier.name).collect();
                let mut has_field_name_errors = false;

                for (arg_ident, _arg_expr) in checked_args.iter() {
                    if decl.fields.iter().any(|p| p.identifier.name == arg_ident.name) {
                        if !uninitialized_field_tracker.remove(&arg_ident.name) {
                            self.errors
                                .push(SemanticError::DuplicateStructFieldInitializer { id: arg_ident.clone() });

                            has_field_name_errors = true;
                        }
                    } else {
                        self.errors
                            .push(SemanticError::UnknownStructFieldInitializer { id: arg_ident.clone() });

                        has_field_name_errors = true;
                    }
                }

                if !uninitialized_field_tracker.is_empty() {
                    self.errors.push(SemanticError::MissingStructFieldInitializer {
                        missing_fields: uninitialized_field_tracker,
                        span,
                    });

                    has_field_name_errors = true;
                };

                if !has_field_name_errors {
                    let mut inferred_substitutions_map = GenericSubstitutionMap::new();
                    if !decl.generic_params.is_empty() {
                        for (arg_ident, arg_expr) in checked_args.iter() {
                            if let Some(field_decl) = decl.fields.iter().find(|p| p.identifier.name == arg_ident.name) {
                                self.infer_generics(&field_decl.constraint, &arg_expr.ty, &mut inferred_substitutions_map);
                            }
                        }
                    }

                    let final_substitutions_opt = self.build_substitution_map(
                        &decl.generic_params,
                        GenericArgumentSource::Inferred {
                            substitutions: &inferred_substitutions_map,
                        },
                        span,
                    );

                    if let Some(final_substitutions) = final_substitutions_opt {
                        let substituted_fields: Vec<CheckedParam> = decl
                            .fields
                            .iter()
                            .map(|p_decl| CheckedParam {
                                identifier: p_decl.identifier,
                                constraint: self.substitute_generics(&p_decl.constraint, &final_substitutions),
                            })
                            .collect();

                        let mut type_mismatch_in_fields = false;
                        for (arg_ident, arg_expr) in &checked_args {
                            if let Some(substituted_field) =
                                substituted_fields.iter().find(|p| p.identifier.name == arg_ident.name)
                            {
                                if !self.check_is_assignable(&arg_expr.ty, &substituted_field.constraint) {
                                    self.errors.push(SemanticError::TypeMismatch {
                                        expected: substituted_field.constraint.clone(),
                                        received: arg_expr.ty.clone(),
                                    });

                                    type_mismatch_in_fields = true;
                                }
                            }
                        }

                        if !type_mismatch_in_fields {
                            result_struct_type = CheckedType {
                                kind: CheckedTypeKind::StructDecl(Rc::new(RefCell::new(CheckedStructDecl {
                                    identifier: decl.identifier,
                                    documentation: decl.documentation.clone(),
                                    fields: substituted_fields,
                                    generic_params: vec![],
                                    span: decl.span,
                                }))),
                                span,
                            }
                        }
                    }
                }
            }
            CheckedTypeKind::Unknown => {}
            _ => {
                self.errors.push(SemanticError::CannotApplyStructInitializer {
                    span: checked_left.ty.span,
                });
            }
        };

        CheckedExpr {
            ty: result_struct_type,
            kind: CheckedExprKind::StructInit {
                left: Box::new(checked_left), // The (checked) expression for the struct type
                fields: checked_args,         // The (checked) field initializers
            },
        }
    }
}
