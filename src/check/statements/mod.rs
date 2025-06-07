use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::{
            base_declaration::{GenericParam, Param, StructDecl, TypeAliasDecl, VarDecl},
            base_statement::{Stmt, StmtKind},
        },
        checked::{
            checked_declaration::{CheckedGenericParam, CheckedParam, CheckedStructDecl, CheckedTypeAliasDecl, CheckedVarDecl},
            checked_expression::{CheckedBlockContents, CheckedExprKind},
            checked_statement::CheckedStmt,
            checked_type::{CheckedType, CheckedTypeKind},
        },
    },
    check::{
        scope::{Scope, ScopeKind, SymbolEntry},
        SemanticChecker, SemanticError,
    },
};

impl<'a> SemanticChecker<'a> {
    pub fn check_generic_params(
        &mut self,
        generic_params: &[GenericParam],
        scope: Rc<RefCell<Scope>>,
    ) -> Vec<CheckedGenericParam> {
        generic_params
            .into_iter()
            .map(|gp| {
                let checked_constraint = gp
                    .constraint
                    .as_ref()
                    .map(|constraint| Box::new(self.check_type(constraint, scope.clone())));

                let checked_gp = CheckedGenericParam {
                    constraint: checked_constraint,
                    identifier: gp.identifier,
                };

                scope
                    .borrow_mut()
                    .insert(gp.identifier.name, SymbolEntry::GenericParam(checked_gp.clone()));
                checked_gp
            })
            .collect()
    }

    pub fn check_struct_fields(&mut self, fields: &Vec<Param>, scope: Rc<RefCell<Scope>>) -> Vec<CheckedParam> {
        fields
            .into_iter()
            .map(|p| CheckedParam {
                constraint: self.check_type(&p.constraint, scope.clone()),
                identifier: p.identifier,
            })
            .collect()
    }

    pub fn check_stmts(&mut self, statements: Vec<Stmt>, scope: Rc<RefCell<Scope>>) -> Vec<CheckedStmt> {
        statements.into_iter().map(|s| self.check_stmt(s, scope.clone())).collect()
    }

    pub fn check_stmt(&mut self, stmt: Stmt, scope: Rc<RefCell<Scope>>) -> CheckedStmt {
        match stmt.kind {
            StmtKind::Expression(expr) => CheckedStmt::Expression(self.check_expr(expr, scope)),
            StmtKind::StructDecl(StructDecl {
                identifier,
                documentation,
                generic_params,
                fields,
            }) => {
                if !scope.borrow().is_file_scope() {
                    self.errors
                        .push(SemanticError::StructMustBeDeclaredAtTopLevel { span: stmt.span });
                }

                let struct_scope = scope.borrow().child(ScopeKind::Struct);

                let generic_params = self.check_generic_params(&generic_params, struct_scope.clone());

                let checked_fields = self.check_struct_fields(&fields, struct_scope.clone());

                let decl = CheckedStructDecl {
                    identifier,
                    documentation,
                    fields: checked_fields,
                    generic_params,
                    span: stmt.span,
                };
                scope
                    .borrow_mut()
                    .insert(identifier.name, SymbolEntry::StructDecl(decl.clone()));

                CheckedStmt::StructDecl(decl)
            }
            StmtKind::EnumDecl(decl) => {
                scope
                    .borrow_mut()
                    .insert(decl.identifier.name, SymbolEntry::EnumDecl(decl.clone()));

                CheckedStmt::EnumDecl(decl)
            }
            StmtKind::VarDecl(VarDecl {
                identifier,
                documentation,
                constraint,
                value,
            }) => {
                let checked_value = value.map(|v| self.check_expr(v, scope.clone()));

                let checked_constraint = constraint.map(|c| self.check_type(&c, scope.clone()));

                let final_constraint = match (&checked_value, checked_constraint) {
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

                let checked_declaration = CheckedVarDecl {
                    documentation,
                    identifier: identifier,
                    constraint: final_constraint,
                    value: checked_value,
                };

                scope
                    .borrow_mut()
                    .insert(identifier.name, SymbolEntry::VarDecl(checked_declaration.clone()));

                CheckedStmt::VarDecl(checked_declaration)
            }
            StmtKind::TypeAliasDecl(TypeAliasDecl {
                identifier,
                documentation,
                generic_params,
                value,
            }) => {
                if !scope.borrow().is_file_scope() {
                    self.errors
                        .push(SemanticError::TypeAliasMustBeDeclaredAtTopLevel { span: stmt.span });
                }

                let alias_scope = scope.borrow().child(ScopeKind::TypeAlias);

                let generic_params = self.check_generic_params(&generic_params, alias_scope.clone());

                let checked_value = self.check_type(&value, alias_scope);

                let decl = CheckedTypeAliasDecl {
                    documentation,
                    identifier,
                    value: Box::new(checked_value),
                    generic_params,
                    span: stmt.span,
                };

                scope
                    .borrow_mut()
                    .insert(identifier.name, SymbolEntry::TypeAliasDecl(decl.clone()));

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

                CheckedStmt::Return(self.check_expr(expr, scope))
            }
            StmtKind::Assignment { target, value } => {
                let checked_target = self.check_expr(target, scope.clone());
                let checked_value = self.check_expr(value, scope.clone());

                match &checked_target.kind {
                    CheckedExprKind::Identifier(id) => {
                        let identifier_expr_type = scope.borrow().lookup(id.name);

                        if let Some(SymbolEntry::VarDecl(decl)) = identifier_expr_type {
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
                            CheckedTypeKind::StructDecl(CheckedStructDecl { fields, .. }) => fields
                                .into_iter()
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
