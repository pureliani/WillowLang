use crate::{
    ast::{expr::Expr, IdentifierNode},
    cfg::Value,
    hir_builder::HIRBuilder,
};

impl<'a> HIRBuilder<'a> {
    pub fn build_struct_initializer_expr(&mut self, fields: Vec<(IdentifierNode, Expr)>) -> Value {
        todo!()
    }
}
