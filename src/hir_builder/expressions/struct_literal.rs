use crate::{
    ast::{expr::Expr, IdentifierNode},
    cfg::Value,
    hir_builder::FunctionBuilder,
};

impl FunctionBuilder {
    pub fn build_struct_initializer_expr(&mut self, fields: Vec<(IdentifierNode, Expr)>) -> Value {
        todo!()
    }
}
