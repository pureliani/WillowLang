use crate::{
    ast::{base::base_expression::Expr, checked::checked_expression::CheckedExpr, IdentifierNode, Span},
    check::SemanticChecker,
};

impl<'a> SemanticChecker<'a> {
    pub fn check_static_access_expr(&mut self, left: Box<Expr>, field: IdentifierNode, span: Span) -> CheckedExpr {
        todo!()
    }
}
