use crate::{
    ast::{expr::Expr, IdentifierNode},
    cfg::Value,
    hir_builder::HIRBuilder,
};

impl<'a> HIRBuilder<'a> {
    pub fn build_struct_literal(&mut self, fields: Vec<(IdentifierNode, Expr)>) -> Value {
        todo!()
    }
}
