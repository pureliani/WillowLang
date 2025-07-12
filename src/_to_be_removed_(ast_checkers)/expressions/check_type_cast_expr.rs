use crate::{
    ast::{
        base::{base_expression::Expr, base_type::TypeAnnotation},
        checked::checked_expression::CheckedExpr,
        Span,
    },
    check::SemanticChecker,
};

impl<'a> SemanticChecker<'a> {
    pub fn check_type_cast_expr(&mut self, left: Box<Expr>, target: TypeAnnotation, span: Span) -> CheckedExpr {
        todo!()
    }
}
