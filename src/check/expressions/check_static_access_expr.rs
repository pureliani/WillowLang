use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{base::base_expression::Expr, checked::checked_expression::CheckedExpr, IdentifierNode},
    check::{scope::Scope, SemanticChecker},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_static_access_expr(
        &mut self,
        left: Box<Expr>,
        field: IdentifierNode,
        scope: Rc<RefCell<Scope>>,
    ) -> CheckedExpr {
        todo!()
    }
}
