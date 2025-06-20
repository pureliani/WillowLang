use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::{
            base_declaration::{StructDecl, TypeAliasDecl, VarDecl},
            base_expression::{Expr, ExprKind},
            base_statement::{Stmt, StmtKind},
        },
        checked::{
            checked_declaration::{CheckedStructDecl, CheckedTypeAliasDecl, CheckedVarDecl},
            checked_expression::{CheckedBlockContents, CheckedExprKind},
            checked_statement::CheckedStmt,
            checked_type::{CheckedType, CheckedTypeKind},
        },
    },
    check::{
        utils::scope::{ScopeKind, SymbolEntry},
        SemanticChecker, SemanticError,
    },
    tfg::TFGNodeKind,
};

impl<'a> SemanticChecker<'a> {
    pub fn placeholder_declarations(&mut self, statements: &Vec<Stmt>) {
        for stmt in statements {
            match &stmt.kind {
                StmtKind::StructDecl(decl) => {
                    let placeholder = SymbolEntry::StructDecl(Rc::new(RefCell::new(CheckedStructDecl {
                        identifier: decl.identifier,
                        documentation: decl.documentation.clone(),
                        fields: vec![],
                        generic_params: vec![],
                        span: decl.identifier.span,
                        applied_type_args: vec![],
                    })));

                    self.scope_insert(decl.identifier, placeholder);
                }
                StmtKind::EnumDecl(decl) => {
                    let actual = SymbolEntry::EnumDecl(decl.clone());

                    self.scope_insert(decl.identifier, actual);
                }
                StmtKind::TypeAliasDecl(decl) => {
                    let placeholder = SymbolEntry::TypeAliasDecl(Rc::new(RefCell::new(CheckedTypeAliasDecl {
                        identifier: decl.identifier,
                        documentation: decl.documentation.clone(),
                        value: Box::new(CheckedType {
                            kind: CheckedTypeKind::Unknown,
                            span: decl.identifier.span,
                        }),
                        generic_params: vec![],
                        applied_type_args: vec![],
                        span: decl.identifier.span,
                    })));

                    self.scope_insert(decl.identifier, placeholder);
                }
                StmtKind::VarDecl(decl) => {
                    if let Some(Expr {
                        kind: ExprKind::Fn { .. },
                        ..
                    }) = &decl.value
                    {
                        let definition_id = self.get_definition_id();
                        let placeholder = SymbolEntry::VarDecl(Rc::new(RefCell::new(CheckedVarDecl {
                            id: definition_id,
                            identifier: decl.identifier,
                            documentation: decl.documentation.clone(),
                            value: None,
                            constraint: CheckedType {
                                kind: CheckedTypeKind::Unknown,
                                span: decl.identifier.span,
                            },
                        })));

                        self.scope_insert(decl.identifier, placeholder);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn check_stmts(&mut self, statements: Vec<Stmt>) -> Vec<CheckedStmt> {
        self.placeholder_declarations(&statements);
        statements
            .into_iter()
            .map(|s| {
                let checked = self.check_stmt(s);
                checked
            })
            .collect()
    }

    pub fn check_stmt(&mut self, stmt: Stmt) -> CheckedStmt {
        match stmt.kind {
            StmtKind::Expression(expr) => CheckedStmt::Expression(self.check_expr(expr)),
            StmtKind::StructDecl(StructDecl {
                identifier,
                generic_params,
                fields,
                documentation: _,
            }) => {
                if !self.is_file_scope() {
                    self.errors
                        .push(SemanticError::StructMustBeDeclaredAtTopLevel { span: stmt.span });
                }

                self.enter_scope(ScopeKind::Struct);
                let checked_generic_params = self.check_generic_params(&generic_params);
                let checked_fields = self.check_params(&fields);
                self.exit_scope();

                let decl = match self.scope_lookup(identifier.name) {
                    Some(SymbolEntry::StructDecl(decl)) => {
                        let mut mut_decl = decl.borrow_mut();
                        mut_decl.fields = checked_fields;
                        mut_decl.generic_params = checked_generic_params;
                        mut_decl.span = stmt.span;
                        decl.clone()
                    }
                    _ => {
                        panic!("Expected struct declaration placeholder")
                    }
                };

                CheckedStmt::StructDecl(decl)
            }
            StmtKind::EnumDecl(decl) => CheckedStmt::EnumDecl(decl.clone()),
            StmtKind::VarDecl(VarDecl {
                identifier,
                constraint,
                value,
                documentation,
            }) => {
                let is_fn = matches!(
                    value,
                    Some(Expr {
                        kind: ExprKind::Fn { .. },
                        ..
                    })
                );

                let constraint = constraint.map(|c| {
                    let checked_constraint = self.check_type_annotation(&c);
                    if is_fn {
                        let placeholder = match self.scope_lookup(identifier.name) {
                            Some(SymbolEntry::VarDecl(d)) => d,
                            _ => panic!("Expected function declaration placeholder for"),
                        };
                        placeholder.borrow_mut().constraint = checked_constraint.clone();
                    };

                    checked_constraint
                });

                let checked_value = value.map(|v| self.check_expr(v));

                let final_constraint = match (&checked_value, constraint) {
                    (Some(value), Some(constraint)) => {
                        let is_assignable = self.check_is_assignable(&value.ty, &constraint);

                        if !is_assignable {
                            self.errors.push(SemanticError::TypeMismatch {
                                expected: constraint.clone(),
                                received: value.ty.clone(),
                            });
                        }

                        constraint
                    }
                    (Some(value), None) => value.ty.clone(),

                    (None, _) => {
                        self.errors.push(SemanticError::VarDeclWithoutInitializer { span: stmt.span });

                        CheckedType {
                            kind: CheckedTypeKind::Unknown,
                            span: identifier.span,
                        }
                    }
                };

                let decl = if is_fn {
                    match self.scope_lookup(identifier.name) {
                        Some(SymbolEntry::VarDecl(decl)) => {
                            let mut mut_decl = decl.borrow_mut();
                            mut_decl.value = checked_value;
                            mut_decl.constraint = final_constraint;
                            decl.clone()
                        }
                        _ => {
                            panic!("Expected function declaration placeholder")
                        }
                    }
                } else {
                    let definition_id = self.get_definition_id();
                    let decl = Rc::new(RefCell::new(CheckedVarDecl {
                        id: definition_id,
                        identifier,
                        documentation,
                        value: checked_value,
                        constraint: final_constraint,
                    }));

                    self.scope_insert(identifier, SymbolEntry::VarDecl(decl.clone()));

                    decl
                };

                if let Some(context) = self.tfg_contexts.last_mut() {
                    if let Some(val) = &decl.borrow().value {
                        let assign_node = context.graph.create_node(TFGNodeKind::Assign {
                            target: decl.borrow().id,
                            assigned_type: Rc::new(val.ty.kind.clone()),
                            next_node: None,
                        });
                        context.graph.link_sequential(context.current_node, assign_node);
                        context.current_node = assign_node;
                    }
                }

                CheckedStmt::VarDecl(decl)
            }
            StmtKind::TypeAliasDecl(TypeAliasDecl {
                identifier,
                generic_params,
                value,
                documentation: _,
            }) => {
                if !self.is_file_scope() {
                    self.errors
                        .push(SemanticError::TypeAliasMustBeDeclaredAtTopLevel { span: stmt.span });
                }

                self.enter_scope(ScopeKind::TypeAlias);
                let checked_generic_params = self.check_generic_params(&generic_params);
                let checked_value = self.check_type_annotation(&value);
                self.exit_scope();

                let decl = match self.scope_lookup(identifier.name) {
                    Some(SymbolEntry::TypeAliasDecl(decl)) => {
                        let mut mut_decl = decl.borrow_mut();
                        mut_decl.value = Box::new(checked_value);
                        mut_decl.generic_params = checked_generic_params;
                        mut_decl.span = stmt.span;
                        decl.clone()
                    }
                    _ => {
                        panic!("Expected type-alias declaration placeholder")
                    }
                };

                CheckedStmt::TypeAliasDecl(decl)
            }
            StmtKind::Break => {
                if !self.within_loop_scope() {
                    self.errors.push(SemanticError::BreakKeywordOutsideLoop { span: stmt.span });
                }

                CheckedStmt::Break { span: stmt.span }
            }
            StmtKind::Continue => {
                if !self.within_loop_scope() {
                    self.errors
                        .push(SemanticError::ContinueKeywordOutsideLoop { span: stmt.span });
                }

                CheckedStmt::Continue { span: stmt.span }
            }
            StmtKind::Return(expr) => {
                if !self.within_function_scope() {
                    self.errors
                        .push(SemanticError::ReturnKeywordOutsideFunction { span: stmt.span });
                }

                let value = self.check_expr(expr);

                if let Some(context) = self.tfg_contexts.last_mut() {
                    let exit_node = context.graph.create_node(TFGNodeKind::Exit);
                    context.graph.link_sequential(context.current_node, exit_node);
                    context.current_node = exit_node;
                }

                CheckedStmt::Return(value)
            }
            StmtKind::Assignment { target, value } => {
                let checked_target = self.check_expr(target);
                let checked_value = self.check_expr(value);

                match &checked_target.kind {
                    CheckedExprKind::Identifier(id) => {
                        let symbol = self.scope_lookup(id.name);

                        if let Some(SymbolEntry::VarDecl(decl)) = symbol {
                            let decl = decl.borrow();

                            let is_assignable = self.check_is_assignable(&checked_value.ty, &decl.constraint);

                            if !is_assignable {
                                self.errors.push(SemanticError::TypeMismatch {
                                    expected: decl.constraint.clone(),
                                    received: checked_value.ty.clone(),
                                });
                            }
                        } else {
                            self.errors.push(SemanticError::UndeclaredIdentifier { id: *id });
                        }
                    }
                    CheckedExprKind::Access { left, field } => {
                        let field_type = match &left.ty.kind {
                            CheckedTypeKind::StructDecl(decl) => decl
                                .borrow()
                                .fields
                                .iter()
                                .find(|p| p.identifier == *field)
                                .map(|p| p.constraint.clone())
                                .unwrap_or_else(|| {
                                    self.errors
                                        .push(SemanticError::AccessToUndefinedField { field: field.clone() });

                                    CheckedType {
                                        kind: CheckedTypeKind::Unknown,
                                        span: field.span,
                                    }
                                }),
                            _ => {
                                self.errors.push(SemanticError::CannotAccess { target: left.ty.clone() });

                                CheckedType {
                                    kind: CheckedTypeKind::Unknown,
                                    span: field.span,
                                }
                            }
                        };

                        let is_assignable = self.check_is_assignable(&checked_value.ty, &field_type);

                        if !is_assignable {
                            self.errors.push(SemanticError::TypeMismatch {
                                expected: field_type,
                                received: checked_value.ty.clone(),
                            });
                        }
                    }
                    _ => {
                        self.errors.push(SemanticError::InvalidAssignmentTarget {
                            target: checked_target.ty.clone(),
                        });
                    }
                }

                CheckedStmt::Assignment {
                    target: checked_target,
                    value: checked_value,
                }
            }
            StmtKind::From { path, identifiers } => CheckedStmt::From {
                identifiers,
                path,
                span: stmt.span,
            },
            StmtKind::While { condition, body } => {
                let mut condition_node = None;
                let mut after_loop_node = None;

                if let Some(context) = self.tfg_contexts.last_mut() {
                    let before_loop_node = context.current_node;

                    let cond_node = context.graph.create_node(TFGNodeKind::NoOp { next_node: None });
                    let after_node = context.graph.create_node(TFGNodeKind::NoOp { next_node: None });

                    condition_node = Some(cond_node);
                    after_loop_node = Some(after_node);

                    context.graph.link_sequential(before_loop_node, cond_node);
                    context.current_node = cond_node;
                }

                let checked_condition = self.check_expr(*condition);

                let expected_condition_type = CheckedType {
                    kind: CheckedTypeKind::Bool,
                    span: checked_condition.ty.span,
                };
                if !self.check_is_assignable(&checked_condition.ty, &expected_condition_type) {
                    self.errors.push(SemanticError::TypeMismatch {
                        expected: expected_condition_type,
                        received: checked_condition.ty.clone(),
                    });
                }

                self.enter_scope(ScopeKind::While);
                if let Some(context) = self.tfg_contexts.last_mut() {
                    let body_node = context.graph.create_node(TFGNodeKind::NoOp { next_node: None });

                    context.graph.build_condition_tfg(
                        &checked_condition,
                        context.current_node,
                        body_node,
                        after_loop_node.unwrap(),
                    );

                    context.current_node = body_node;
                    context.loop_exit_nodes.push(after_loop_node.unwrap());
                }

                let checked_body_statements = self.check_stmts(body.statements);
                let checked_final_expr = body.final_expr.map(|expr| Box::new(self.check_expr(*expr)));
                self.exit_scope();

                if after_loop_node.is_some() {
                    if let Some(context) = self.tfg_contexts.last_mut() {
                        context.loop_exit_nodes.pop();
                    }
                }

                if let Some(context) = self.tfg_contexts.last_mut() {
                    context.graph.link_sequential(context.current_node, condition_node.unwrap());
                    context.current_node = after_loop_node.unwrap();
                }

                CheckedStmt::While {
                    condition: Box::new(checked_condition),
                    body: CheckedBlockContents {
                        final_expr: checked_final_expr,
                        statements: checked_body_statements,
                    },
                    span: stmt.span,
                }
            }
        }
    }
}
