use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::checked_expression::{CheckedExpr, CheckedExprKind},
        IdentifierNode,
    },
    check::{check_expr::check_expr, scope::Scope, SemanticError},
};

pub fn check_access_expr(
    left: Box<Expr>,
    field: IdentifierNode,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_left = check_expr(*left, errors, scope);

    // TODO: implement this

    CheckedExpr {
        kind: CheckedExprKind::Access {
            left: Box::new(checked_left.clone()),
            field: field.clone(),
        },
        expr_type,
    }
}
