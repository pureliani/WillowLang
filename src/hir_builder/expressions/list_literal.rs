use crate::{
    ast::expr::Expr,
    cfg::Value,
    hir_builder::{FunctionBuilder, HIRContext},
};

impl FunctionBuilder {
    pub fn build_list_literal_expr(&mut self, ctx: &mut HIRContext, items: Vec<Expr>) -> Value {
        todo!()
    }
}
