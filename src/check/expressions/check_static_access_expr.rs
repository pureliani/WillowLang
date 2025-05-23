use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{base::base_expression::Expr, checked::checked_expression::CheckedExpr, IdentifierNode},
    check::{scope::Scope, SemanticError},
};

pub fn check_static_access_expr(
    left: Box<Expr>,
    field: IdentifierNode,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    todo!()
}
