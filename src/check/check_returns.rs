use std::{cell::RefCell, rc::Rc};

use crate::ast::checked::{
    checked_expression::{CheckedExpr, CheckedExprKind},
    checked_statement::{CheckedStmt, CheckedStmtKind},
};

use super::{scope::Scope, SemanticError, SemanticErrorKind};

pub fn check_returns(
    statements: &[CheckedStmt],
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> Vec<CheckedExpr> {
    let mut returns: Vec<CheckedExpr> = vec![];

    let stmt_count = statements.len();

    for (i, stmt) in statements.iter().enumerate() {
        match &stmt.kind {
            CheckedStmtKind::Return(expr) => {
                if i < stmt_count - 1 {
                    errors.push(SemanticError::new(
                        SemanticErrorKind::ReturnNotLastStatement,
                        stmt.span,
                    ));
                }
                returns.push(expr.clone());
            }
            CheckedStmtKind::While { body, .. } => {
                returns.extend(check_returns(&body.statements, errors, scope.clone()));
            }
            CheckedStmtKind::Expression(expr) => {
                if let CheckedExprKind::If {
                    then_branch,
                    else_if_branches,
                    else_branch,
                    ..
                } = &expr.kind
                {
                    returns.extend(check_returns(
                        &then_branch.statements,
                        errors,
                        scope.clone(),
                    ));

                    for (_, block) in else_if_branches {
                        returns.extend(check_returns(&block.statements, errors, scope.clone()));
                    }
                    if let Some(else_block) = else_branch {
                        returns.extend(check_returns(
                            &else_block.statements,
                            errors,
                            scope.clone(),
                        ));
                    }
                } else if let CheckedExprKind::Block(block) = &expr.kind {
                    returns.extend(check_returns(&block.statements, errors, scope.clone()));
                    if let Some(final_expr) = &block.final_expr {
                        returns.push(*final_expr.clone());
                    }
                }
            }
            _ => (),
        }
    }

    returns
}
