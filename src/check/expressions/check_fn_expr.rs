use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::{
            base_declaration::{GenericParam, Param},
            base_expression::BlockContents,
            base_type::TypeAnnotation,
        },
        checked::{
            checked_declaration::{CheckedParam, CheckedVarDecl},
            checked_expression::{CheckedBlockContents, CheckedExpr, CheckedExprKind},
            checked_type::CheckedTypeKind,
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
                let checked_constraint = self.check_type(&param.constraint, fn_scope.clone());
                let span = Span {
                    start: param.identifier.span.start,
                    end: param.constraint.span.end,
                };

                fn_scope.borrow_mut().insert(
                    param.identifier.name,
                    SymbolEntry::VarDecl(CheckedVarDecl {
                        documentation: None,
                        identifier: param.identifier,
                        constraint: checked_constraint.clone(),
                        value: None,
                        span,
                    }),
                );

                CheckedParam {
                    constraint: checked_constraint,
                    identifier: param.identifier,
                }
            })
            .collect();

        self.check_codeblock_expr(body, body);
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

        let inferred_return_type = if return_exprs.len() > 1 {
            union_of(return_exprs.iter().map(|e| e.ty.clone()))
        } else if return_exprs.len() == 1 {
            return_exprs.get(0).map(|e| e.ty.clone()).unwrap()
        } else {
            CheckedTypeKind::Void { node_id }
        };

        let param_types: Vec<CheckedParam> = params
            .into_iter()
            .map(|p| CheckedParam {
                constraint: self.check_type(&p.constraint, fn_scope.clone()),
                identifier: p.identifier,
            })
            .collect();

        let expected_return_type = return_type.map(|return_t| self.check_type(&return_t, fn_scope.clone()));

        let actual_return_type = if let Some(explicit_return_type) = expected_return_type {
            if !self.check_is_assignable(&inferred_return_type, &explicit_return_type) {
                self.errors.push(SemanticError {
                    kind: SemanticErrorKind::ReturnTypeMismatch {
                        expected: explicit_return_type.clone(),
                        received: inferred_return_type.clone(),
                    },
                    span,
                });
            }

            explicit_return_type
        } else {
            inferred_return_type
        };

        let expr_type = CheckedTypeKind::FnType {
            params: param_types,
            return_type: Box::new(actual_return_type.clone()),
            generic_params: checked_generic_params.clone(),
            node_id,
        };

        CheckedExpr {
            node_id,
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
