use std::{cell::RefCell, rc::Rc};

use crate::ast::{
    base::{
        base_declaration::{GenericParam, Param, StructDecl, TypeAliasDecl, VarDecl},
        base_statement::{Stmt, StmtKind},
    },
    checked::{
        checked_declaration::{
            CheckedGenericParam, CheckedParam, CheckedStructDecl, CheckedVarDecl,
        },
        checked_expression::{CheckedBlockContents, CheckedExprKind},
        checked_statement::{CheckedStmt, CheckedStmtKind},
        checked_type::{Type, TypeKind, TypeSpan},
    },
};

use super::{
    check_expr::check_expr,
    check_stmts::check_stmts,
    scope::{Scope, ScopeKind, SymbolEntry},
    utils::{
        check_is_assignable::check_is_assignable,
        type_annotation_to_semantic::type_annotation_to_semantic,
    },
    SemanticError, SemanticErrorKind,
};

pub fn check_generic_params(
    generic_params: &Vec<GenericParam>,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> Vec<CheckedGenericParam> {
    generic_params
        .into_iter()
        .map(|gp| {
            let checked_constraint = gp.constraint.as_ref().map(|constraint| {
                Box::new(type_annotation_to_semantic(
                    constraint,
                    errors,
                    scope.clone(),
                ))
            });

            let checked_gp = CheckedGenericParam {
                constraint: checked_constraint,
                identifier: gp.identifier.clone(),
            };

            scope.borrow_mut().insert(
                gp.identifier.name.clone(),
                SymbolEntry::GenericParam(checked_gp.clone()),
            );
            checked_gp
        })
        .collect()
}

pub fn check_struct_properties(
    properties: &Vec<Param>,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> Vec<CheckedParam> {
    properties
        .into_iter()
        .map(|p| CheckedParam {
            constraint: type_annotation_to_semantic(&p.constraint, errors, scope.clone()),
            identifier: p.identifier.to_owned(),
        })
        .collect()
}

pub fn check_stmt(
    stmt: Stmt,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedStmt {
    match stmt.kind {
        StmtKind::Expression(expr) => CheckedStmt {
            kind: CheckedStmtKind::Expression(check_expr(expr, errors, scope)),
            span: stmt.span,
        },
        StmtKind::StructDecl(StructDecl {
            identifier,
            documentation,
            generic_params,
            properties,
        }) => {
            let struct_scope = scope.borrow().child(ScopeKind::Struct);

            let generic_params =
                check_generic_params(&generic_params, errors, struct_scope.clone());

            let checked_properties =
                check_struct_properties(&properties, errors, struct_scope.clone());

            let checked_declaration = CheckedStructDecl {
                identifier: identifier.to_owned(),
                documentation,
                properties: checked_properties,
                generic_params,
            };

            scope.borrow_mut().insert(
                identifier.name,
                SymbolEntry::StructDecl(checked_declaration.clone()),
            );

            CheckedStmt {
                kind: CheckedStmtKind::StructDecl(checked_declaration),
                span: stmt.span,
            }
        }
        StmtKind::EnumDecl(decl) => {
            scope.borrow_mut().insert(
                decl.identifier.name.clone(),
                SymbolEntry::EnumDecl(decl.clone()),
            );

            CheckedStmt {
                kind: CheckedStmtKind::EnumDecl(decl),
                span: stmt.span,
            }
        }
        StmtKind::VarDecl(VarDecl {
            identifier,
            documentation,
            constraint,
            value,
        }) => {
            let checked_value = value.map(|v| check_expr(v, errors, scope.clone()));

            let checked_constraint =
                constraint.map(|c| type_annotation_to_semantic(&c, errors, scope.clone()));

            let final_constraint = match (&checked_value, checked_constraint) {
                (None, None) => {
                    errors.push(SemanticError::new(
                        SemanticErrorKind::VarDeclWithNoConstraintOrInitializer,
                        stmt.span,
                    ));

                    Type {
                        kind: TypeKind::Unknown,
                        span: TypeSpan::Annotation(identifier.span),
                    }
                }
                (Some(value), Some(constraint)) => {
                    let is_assignable = check_is_assignable(&value.expr_type, &constraint);

                    if !is_assignable {
                        errors.push(SemanticError::new(
                            SemanticErrorKind::TypeMismatch {
                                expected: constraint.clone(),
                                received: value.expr_type.clone(),
                            },
                            stmt.span,
                        ));
                    }

                    constraint
                }
                (Some(value), None) => value.expr_type.clone(),
                (None, Some(t)) => t.clone(),
            };

            let checked_declaration = CheckedVarDecl {
                documentation,
                identifier: identifier.to_owned(),
                constraint: final_constraint,
                value: checked_value,
            };

            scope.borrow_mut().insert(
                identifier.name,
                SymbolEntry::VarDecl(checked_declaration.clone()),
            );

            CheckedStmt {
                kind: CheckedStmtKind::VarDecl(checked_declaration),
                span: stmt.span,
            }
        }
        StmtKind::TypeAliasDecl(TypeAliasDecl {
            identifier,
            documentation,
            generic_params,
            value,
        }) => todo!(),
        StmtKind::Break => {
            if !scope.borrow().is_within_loop() {
                errors.push(SemanticError::new(
                    SemanticErrorKind::BreakKeywordOutsideLoop,
                    stmt.span,
                ));
            }

            CheckedStmt {
                kind: CheckedStmtKind::Break,
                span: stmt.span,
            }
        }
        StmtKind::Continue => {
            if !scope.borrow().is_within_loop() {
                errors.push(SemanticError::new(
                    SemanticErrorKind::ContinueKeywordOutsideLoop,
                    stmt.span,
                ));
            }

            CheckedStmt {
                kind: CheckedStmtKind::Continue,
                span: stmt.span,
            }
        }
        StmtKind::Return(expr) => {
            if !scope.borrow().is_within_function() {
                errors.push(SemanticError::new(
                    SemanticErrorKind::ReturnKeywordOutsideFunction,
                    stmt.span,
                ));
            }

            CheckedStmt {
                kind: CheckedStmtKind::Return(check_expr(expr, errors, scope)),
                span: stmt.span,
            }
        }
        StmtKind::Assignment { target, value } => {
            let checked_target = check_expr(target, errors, scope.clone());
            let checked_value = check_expr(value, errors, scope.clone());

            match &checked_target.kind {
                CheckedExprKind::Identifier(id) => {
                    let identifier_expr_type = scope.borrow().lookup(&id.name);

                    if let Some(SymbolEntry::VarDecl(decl)) = identifier_expr_type {
                        let is_assignable =
                            check_is_assignable(&checked_value.expr_type, &decl.constraint);

                        if !is_assignable {
                            errors.push(SemanticError::new(
                                SemanticErrorKind::TypeMismatch {
                                    expected: decl.constraint.clone(),
                                    received: checked_value.expr_type.clone(),
                                },
                                stmt.span,
                            ));
                        }
                    } else {
                        errors.push(SemanticError::new(
                            SemanticErrorKind::UndeclaredIdentifier(id.name.clone()),
                            checked_target.expr_type.unwrap_expr_span(),
                        ));
                    }
                }
                CheckedExprKind::Access { left, field } => {}
                _ => {
                    errors.push(SemanticError::new(
                        SemanticErrorKind::InvalidAssignmentTarget,
                        checked_target.expr_type.unwrap_expr_span(),
                    ));
                }
            }

            CheckedStmt {
                kind: CheckedStmtKind::Assignment {
                    target: checked_target,
                    value: checked_value,
                },
                span: stmt.span,
            }
        }
        StmtKind::From { path, identifiers } => CheckedStmt {
            kind: CheckedStmtKind::From { identifiers, path },
            span: stmt.span,
        },
        StmtKind::While { condition, body } => {
            let while_scope = scope.borrow().child(ScopeKind::While);

            let checked_condition = check_expr(*condition, errors, scope.clone());

            if checked_condition.expr_type.kind != TypeKind::Bool {
                errors.push(SemanticError::new(
                    SemanticErrorKind::TypeMismatch {
                        expected: Type {
                            kind: TypeKind::Bool,
                            span: checked_condition.expr_type.span,
                        },
                        received: checked_condition.expr_type.clone(),
                    },
                    checked_condition.expr_type.unwrap_expr_span(),
                ));
            }

            let checked_final_expr = body
                .final_expr
                .map(|expr| Box::new(check_expr(*expr, errors, while_scope.clone())));

            let checked_body_statements = check_stmts(body.statements, errors, while_scope);

            CheckedStmt {
                kind: CheckedStmtKind::While {
                    condition: Box::new(checked_condition),
                    body: CheckedBlockContents {
                        final_expr: checked_final_expr,
                        statements: checked_body_statements,
                    },
                },
                span: stmt.span,
            }
        }
        StmtKind::Error(parsing_error) => todo!(),
    }
}
