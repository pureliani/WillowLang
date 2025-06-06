use std::{cell::RefCell, collections::HashSet, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_declaration::{CheckedParam, CheckedStructDecl},
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::CheckedTypeKind,
        },
        IdentifierNode, Span,
    },
    check::{
        scope::Scope, utils::substitute_generics::GenericSubstitutionMap, SemanticChecker, SemanticError, SemanticErrorKind,
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
        let node_id = self.get_node_id();
        self.span_registry.insert_span(node_id, span);

        let checked_left = self.check_expr(*left_expr, scope.clone());

        let checked_args: Vec<(IdentifierNode, CheckedExpr)> = fields
            .into_iter()
            .map(|(ident, expr)| (ident, self.check_expr(expr, scope.clone())))
            .collect();

        // TODO: make sure that checked_left.kind is actually an identifier or genericapply wrapper around an identifier

        match &checked_left.ty {
            CheckedTypeKind::StructDecl { decl, .. } => {
                let mut substitutions = GenericSubstitutionMap::new();
                let mut uninitialized_props: HashSet<InternerId> = decl.properties.iter().map(|p| p.identifier.name).collect();

                for (arg_ident, arg_expr) in checked_args.iter() {
                    let prop = decl.properties.iter().find(|p| p.identifier.name == arg_ident.name);

                    match prop {
                        None => {
                            self.errors.push(SemanticError {
                                kind: SemanticErrorKind::UnknownStructPropertyInitializer(arg_ident.clone()),
                                span: arg_ident.span,
                            });
                        }
                        Some(prop) => {
                            let already_initialized = !uninitialized_props.remove(&arg_ident.name);
                            if already_initialized {
                                self.errors.push(SemanticError {
                                    kind: SemanticErrorKind::DuplicateStructPropertyInitializer(arg_ident.clone()),
                                    span: arg_ident.span,
                                });
                            } else {
                                self.infer_generics(&prop.constraint, &arg_expr.ty, &mut substitutions);
                            }
                        }
                    }
                }

                let final_properties: Vec<CheckedParam> = decl
                    .properties
                    .iter()
                    .map(|p| CheckedParam {
                        identifier: p.identifier, // Keep original identifier node from declaration
                        constraint: self.substitute_generics(&p.constraint, &substitutions),
                    })
                    .collect();

                for (arg_ident, arg_expr) in &checked_args {
                    if let Some(final_prop) = final_properties.iter().find(|p| p.identifier.name == arg_ident.name) {
                        if !self.check_is_assignable(&arg_expr.ty, &final_prop.constraint) {
                            self.errors.push(SemanticError {
                                kind: SemanticErrorKind::TypeMismatch {
                                    expected: final_prop.constraint.clone(),
                                    received: arg_expr.ty.clone(),
                                },
                                span: arg_expr.span,
                            });
                        }
                    }
                }

                if !uninitialized_props.is_empty() {
                    self.errors.push(SemanticError {
                        kind: SemanticErrorKind::MissingStructPropertyInitializer(uninitialized_props),
                        span,
                    });
                }

                let final_struct_type = CheckedTypeKind::StructDecl {
                    decl: CheckedStructDecl {
                        identifier: decl.identifier,
                        documentation: decl.documentation.clone(),
                        properties: final_properties,
                        generic_params: vec![],
                    },
                    node_id,
                };

                CheckedExpr {
                    ty: final_struct_type,
                    kind: CheckedExprKind::StructInit {
                        left: Box::new(checked_left),
                        fields: checked_args,
                    },
                    span,
                }
            }
            _ => {
                self.errors.push(SemanticError {
                    kind: SemanticErrorKind::CannotApplyStructInitializer,
                    span: checked_left.span,
                });
                CheckedExpr {
                    ty: CheckedTypeKind::Unknown { node_id },
                    kind: CheckedExprKind::StructInit {
                        left: Box::new(checked_left),
                        fields: checked_args,
                    },
                    span,
                }
            }
        }
    }
}
