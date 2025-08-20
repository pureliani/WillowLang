use crate::{ast::expr::Expr, cfg::Value, hir_builder::HIRBuilder};

impl<'a> HIRBuilder<'a> {
    pub fn build_and_expr(&mut self, left: Box<Expr>, right: Box<Expr>) -> Value {
        todo!()
    }
}
