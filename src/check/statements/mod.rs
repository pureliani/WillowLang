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
        scope::{Scope, ScopeKind, SymbolEntry},
        SemanticChecker, SemanticError,
    },
    tfg::TFGNodeKind,
};

impl<'a> SemanticChecker<'a> {
    pub fn placeholder_declarations(&mut self, statements: &Vec<Stmt>, scope: Rc<RefCell<Scope>>) {
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

                    scope.borrow_mut().insert(decl.identifier, placeholder, self.errors);
                }
                StmtKind::EnumDecl(decl) => {
                    let actual = SymbolEntry::EnumDecl(Rc::new(RefCell::new(decl.clone())));

                    scope.borrow_mut().insert(decl.identifier, actual, self.errors);
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

                    scope.borrow_mut().insert(decl.identifier, placeholder, self.errors);
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

                        scope.borrow_mut().insert(decl.identifier, placeholder, self.errors);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn check_stmts(&mut self, statements: Vec<Stmt>, scope: Rc<RefCell<Scope>>) -> Vec<CheckedStmt> {
        self.placeholder_declarations(&statements, scope.clone());
        statements.into_iter().map(|s| self.check_stmt(s, scope.clone())).collect()
    }

    pub fn check_stmt(&mut self, stmt: Stmt, scope: Rc<RefCell<Scope>>) -> CheckedStmt {
        match stmt.kind {
            StmtKind::Expression(expr) => CheckedStmt::Expression(self.check_expr(expr, scope)),
            StmtKind::StructDecl(StructDecl {
                identifier,
                generic_params,
                fields,
                documentation: _,
            }) => {
                if !scope.borrow().is_file_scope() {
                    self.errors
                        .push(SemanticError::StructMustBeDeclaredAtTopLevel { span: stmt.span });
                }

                let struct_scope = scope.borrow().child(ScopeKind::Struct);

                let checked_generic_params = self.check_generic_params(&generic_params, struct_scope.clone());
                let checked_fields = self.check_params(&fields, struct_scope.clone());

                let decl = match scope.borrow_mut().lookup(identifier.name) {
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
            StmtKind::EnumDecl(decl) => {
                let decl = match scope.borrow().lookup(decl.identifier.name) {
                    Some(SymbolEntry::EnumDecl(enum_decl)) => enum_decl,
                    _ => panic!("Expected enum declaration"),
                };

                CheckedStmt::EnumDecl(decl)
            }
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
                    let checked_constraint = self.check_type_annotation(&c, scope.clone());
                    if is_fn {
                        let placeholder_ref = match scope.borrow().lookup(identifier.name) {
                            Some(SymbolEntry::VarDecl(d)) => d,
                            _ => panic!("Expected function declaration placeholder for"),
                        };
                        placeholder_ref.borrow_mut().constraint = checked_constraint.clone();
                    };

                    checked_constraint
                });

                let checked_value = value.map(|v| self.check_expr(v, scope.clone()));

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
                    match scope.borrow_mut().lookup(identifier.name) {
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

                    scope
                        .borrow_mut()
                        .insert(identifier, SymbolEntry::VarDecl(decl.clone()), self.errors);

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
                if !scope.borrow().is_file_scope() {
                    self.errors
                        .push(SemanticError::TypeAliasMustBeDeclaredAtTopLevel { span: stmt.span });
                }

                let alias_scope = scope.borrow().child(ScopeKind::TypeAlias);

                let checked_generic_params = self.check_generic_params(&generic_params, alias_scope.clone());

                let checked_value = self.check_type_annotation(&value, alias_scope);

                let decl = match scope.borrow_mut().lookup(identifier.name) {
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
                if !scope.borrow().is_loop_scope() {
                    self.errors.push(SemanticError::BreakKeywordOutsideLoop { span: stmt.span });
                }

                CheckedStmt::Break { span: stmt.span }
            }
            StmtKind::Continue => {
                if !scope.borrow().is_loop_scope() {
                    self.errors
                        .push(SemanticError::ContinueKeywordOutsideLoop { span: stmt.span });
                }

                CheckedStmt::Continue { span: stmt.span }
            }
            StmtKind::Return(expr) => {
                if !scope.borrow().is_function_scope() {
                    self.errors
                        .push(SemanticError::ReturnKeywordOutsideFunction { span: stmt.span });
                }

                let value = self.check_expr(expr, scope);

                if let Some(context) = self.tfg_contexts.last_mut() {
                    let exit_node = context.graph.create_node(TFGNodeKind::Exit);
                    context.graph.link_sequential(context.current_node, exit_node);
                    context.current_node = exit_node;
                }

                CheckedStmt::Return(value)
            }
            StmtKind::Assignment { target, value } => {
                let checked_target = self.check_expr(target, scope.clone());
                let checked_value = self.check_expr(value, scope.clone());

                match &checked_target.kind {
                    CheckedExprKind::Identifier(id) => {
                        let identifier_expr_type = scope.borrow().lookup(id.name);

                        if let Some(SymbolEntry::VarDecl(decl)) = identifier_expr_type {
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
                let while_scope = scope.borrow().child(ScopeKind::While);

                let checked_condition = self.check_expr(*condition, scope.clone());
                let expected_condition = CheckedType {
                    kind: CheckedTypeKind::Bool,
                    span: checked_condition.ty.span,
                };

                if !self.check_is_assignable(&checked_condition.ty, &expected_condition) {
                    self.errors.push(SemanticError::TypeMismatch {
                        expected: expected_condition,
                        received: checked_condition.ty.clone(),
                    });
                }

                let checked_final_expr = body
                    .final_expr
                    .map(|expr| Box::new(self.check_expr(*expr, while_scope.clone())));

                let checked_body_statements = self.check_stmts(body.statements, while_scope);

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
