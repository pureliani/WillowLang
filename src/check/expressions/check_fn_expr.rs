use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::{
            base_declaration::{GenericParam, Param},
            base_expression::BlockContents,
            base_type::TypeAnnotation,
        },
        checked::{
            checked_declaration::{CheckedFnType, CheckedParam, CheckedVarDecl},
            checked_expression::{CheckedBlockContents, CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
        Span,
    },
    check::{
        scope::{Scope, ScopeKind, SymbolEntry},
        utils::union_of::union_of,
        SemanticChecker, SemanticError,
    },
};

impl<'a> SemanticChecker<'a> {
    pub fn check_fn_expr(
        &mut self,
        params: Vec<Param>,
        body: BlockContents,
        return_type: Option<TypeAnnotation>,
        generic_params: Vec<GenericParam>,
        span: Span,
        scope: Rc<RefCell<Scope>>,
    ) -> CheckedExpr {
        let fn_scope = scope.borrow().child(ScopeKind::Function);

        let checked_generic_params = self.check_generic_params(&generic_params, fn_scope.clone());

        let checked_params: Vec<CheckedParam> = params
            .iter()
            .map(|param| {
                let checked_constraint = self.check_type_annotation_recursive(&param.constraint, fn_scope.clone());

                fn_scope.borrow_mut().insert(
                    param.identifier,
                    SymbolEntry::VarDecl(Rc::new(RefCell::new(CheckedVarDecl {
                        documentation: None,
                        identifier: param.identifier,
                        constraint: checked_constraint.clone(),
                        value: None,
                    }))),
                    self.errors,
                );

                CheckedParam {
                    constraint: checked_constraint,
                    identifier: param.identifier,
                }
            })
            .collect();

        let checked_statements = self.check_stmts(body.statements, fn_scope.clone());
        let checked_final_expr = body.final_expr.map(|fe| Box::new(self.check_expr(*fe, fn_scope.clone())));

        let checked_body = CheckedBlockContents {
            statements: checked_statements.clone(),
            final_expr: checked_final_expr.clone(),
        };

        let mut return_exprs = self.check_returns(&checked_statements, fn_scope.clone());
        if let Some(final_expr) = checked_final_expr {
            return_exprs.push(*final_expr);
        }

        let actual_return_type = if return_exprs.len() > 1 {
            union_of(return_exprs.iter().map(|e| e.ty.clone()), span)
        } else if return_exprs.len() == 1 {
            return_exprs.get(0).map(|e| e.ty.clone()).unwrap()
        } else {
            CheckedType {
                kind: CheckedTypeKind::Void,
                span,
            }
        };

        let expected_return_type = return_type.map(|return_t| self.check_type_annotation_recursive(&return_t, fn_scope.clone()));

        let actual_return_type = if let Some(explicit_return_type) = expected_return_type {
            if !self.check_is_assignable(&actual_return_type, &explicit_return_type) {
                self.errors.push(SemanticError::ReturnTypeMismatch {
                    expected: explicit_return_type.clone(),
                    received: actual_return_type.clone(),
                });
            }

            explicit_return_type
        } else {
            actual_return_type
        };

        let expr_type = CheckedType {
            kind: CheckedTypeKind::FnType(CheckedFnType {
                params: checked_params.clone(),
                return_type: Box::new(actual_return_type.clone()),
                generic_params: checked_generic_params.clone(),
                span,
            }),
            span,
        };

        CheckedExpr {
            ty: expr_type,
            kind: CheckedExprKind::Fn {
                params: checked_params,
                body: checked_body,
                return_type: actual_return_type,
                generic_params: checked_generic_params,
            },
        }
    }
}
