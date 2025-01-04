use std::{cell::RefCell, rc::Rc};

use crate::ast::{base::base_statement::Stmt, checked::checked_statement::CheckedStmt};

use super::{check_stmt::check_stmt, scope::Scope, SemanticError};

pub fn check_stmts(
    statements: Vec<Stmt>,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> Vec<CheckedStmt> {
    let mut result = vec![];

    for stmt in statements {
        result.push(check_stmt(stmt, errors, scope.clone()));
    }

    result
}
