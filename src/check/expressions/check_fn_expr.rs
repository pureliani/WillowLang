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
        utils::{
            scope::{ScopeKind, SymbolEntry},
            union_of::union_of,
        },
        SemanticChecker, SemanticError, TFGContext,
    },
    // Make sure TFGNodeKind and TypeFlowGraph are imported
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
    ) -> CheckedExpr {
        let fn_definition_id = self.get_definition_id();
        self.enter_scope(ScopeKind::Function);

        let checked_generic_params = self.check_generic_params(&generic_params);

        let new_tfg = TypeFlowGraph::new();
        let entry_node_id = new_tfg.entry_node_id;
        self.tfg_contexts.push(TFGContext {
            loop_exit_nodes: vec![],
            graph: new_tfg,
            current_node: entry_node_id,
        });

        let checked_params: Vec<CheckedParam> = params
            .iter()
            .map(|param| {
                let id = self.get_definition_id();
                let checked_constraint = self.check_type_annotation(&param.constraint);

                self.scope_insert(
                    param.identifier,
                    SymbolEntry::VarDecl(Rc::new(RefCell::new(CheckedVarDecl {
                        id,
                        documentation: None,
                        identifier: param.identifier,
                        constraint: checked_constraint.clone(),
                        value: None,
                    }))),
                );

                CheckedParam {
                    id,
                    constraint: checked_constraint,
                    identifier: param.identifier,
                }
            })
            .collect();

        if let Some(context) = self.tfg_contexts.last_mut() {
            let entry_node = context.graph.get_node_mut(context.graph.entry_node_id).unwrap();
            for param in &checked_params {
                entry_node
                    .variable_types
                    .insert(param.id, Rc::new(param.constraint.kind.clone()));
            }
        }

        let checked_statements = self.check_stmts(body.statements);
        let checked_final_expr = body.final_expr.map(|fe| Box::new(self.check_expr(*fe)));

        let checked_body = CheckedBlockContents {
            statements: checked_statements.clone(),
            final_expr: checked_final_expr.clone(),
        };

        let mut return_exprs = self.check_returns(&checked_statements);
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

        let expected_return_type = return_type.map(|return_t| self.check_type_annotation(&return_t));

        let final_return_type = if let Some(explicit_return_type) = expected_return_type {
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

        let current_node_id = completed_context.current_node;
        let needs_exit_node = if let Some(node) = completed_context.graph.get_node(current_node_id) {
            !matches!(node.kind, TFGNodeKind::Exit)
        } else {
            false
        };

        if needs_exit_node {
            let exit_node = completed_context.graph.create_node(TFGNodeKind::Exit);
            completed_context.graph.link_successor(current_node_id, exit_node);
        }

        let summary = completed_context.graph.generate_summary();

        let expr_type = CheckedType {
            kind: CheckedTypeKind::FnType(CheckedFnType {
                params: checked_params.clone(),
                return_type: Box::new(final_return_type.clone()),
                generic_params: checked_generic_params.clone(),
                applied_type_args: vec![],
                span,
            }),
            span,
        };
        self.exit_scope();

        CheckedExpr {
            ty: expr_type,
            kind: CheckedExprKind::Fn {
                id: fn_definition_id,
                params: checked_params,
                body: checked_body,
                return_type: final_return_type,
                generic_params: checked_generic_params,
                tfg: completed_context.graph,
                summary,
            },
        }
    }
}
