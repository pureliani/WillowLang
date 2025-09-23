use crate::{
    ast::expr::Expr,
    hir::{cfg::Value, FunctionBuilder, HIRContext},
};

impl FunctionBuilder {
    pub fn build_list_literal_expr(&mut self, ctx: &mut HIRContext, items: Vec<Expr>) -> Value {
        todo!()
    }
}
