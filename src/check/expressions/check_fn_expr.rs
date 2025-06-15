use std::{cell::RefCell, rc::Rc, vec};

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
        SemanticChecker, SemanticError, TFGContext,
    },
    tfg::{TFGNodeKind, TypeFlowGraph},
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
        let fn_definition_id = self.get_definition_id();
        let fn_scope = scope.borrow().child(ScopeKind::Function);

        let checked_generic_params = self.check_generic_params(&generic_params, fn_scope.clone());

        let new_tfg = TypeFlowGraph::new();
        let entry_node = new_tfg.entry_node_id;
        self.tfg_contexts.push(TFGContext {
            graph: new_tfg,
            current_node: entry_node,
        });

        let checked_params: Vec<CheckedParam> = params
            .iter()
            .map(|param| {
                let id = self.get_definition_id();
                let checked_constraint = self.check_type_annotation(&param.constraint, fn_scope.clone());

                fn_scope.borrow_mut().insert(
                    param.identifier,
                    SymbolEntry::VarDecl(Rc::new(RefCell::new(CheckedVarDecl {
                        id,
                        documentation: None,
                        identifier: param.identifier,
                        constraint: checked_constraint.clone(),
                        value: None,
                    }))),
                    self.errors,
                );

                if let Some(context) = self.tfg_contexts.last_mut() {
                    let entry_node = context.graph.get_node_mut(context.graph.entry_node_id).unwrap();
                    entry_node.variable_types.insert(id, Rc::new(checked_constraint.kind.clone()));
                }

                CheckedParam {
                    id,
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

        let expected_return_type = return_type.map(|return_t| self.check_type_annotation(&return_t, fn_scope.clone()));

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

        let mut completed_context = self.tfg_contexts.pop().expect("TFG context stack should not be empty");

        let current_node_kind = completed_context
            .graph
            .get_node(completed_context.current_node)
            .map(|n| &n.kind);

        if !matches!(current_node_kind, Some(TFGNodeKind::Exit)) {
            let exit_node = completed_context.graph.create_node(TFGNodeKind::Exit);
            completed_context
                .graph
                .link_sequential(completed_context.current_node, exit_node);
        }

        let summary = completed_context.graph.generate_summary();

        let expr_type = CheckedType {
            kind: CheckedTypeKind::FnType(CheckedFnType {
                params: checked_params.clone(),
                return_type: Box::new(actual_return_type.clone()),
                generic_params: checked_generic_params.clone(),
                applied_type_args: vec![],
                span,
            }),
            span,
        };

        CheckedExpr {
            ty: expr_type,
            kind: CheckedExprKind::Fn {
                id: fn_definition_id,
                params: checked_params,
                body: checked_body,
                return_type: actual_return_type,
                generic_params: checked_generic_params,
                tfg: completed_context.graph,
                summary,
            },
        }
    }
}
