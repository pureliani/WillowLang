use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{base::base_expression::Expr, checked::checked_expression::CheckedExpr, IdentifierNode, Span},
    check::{scope::Scope, SemanticChecker},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_static_access_expr(
        &mut self,
        left: Box<Expr>,
        field: IdentifierNode,
        span: Span,
        scope: Rc<RefCell<Scope>>,
    ) -> CheckedExpr {
        let node_id = self.get_node_id();
        self.span_registry.insert_span(node_id, span);
        todo!()
    }
}
