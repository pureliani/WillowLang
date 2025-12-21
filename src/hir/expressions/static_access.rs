use crate::{
    ast::{expr::Expr, IdentifierNode},
    hir::{cfg::Value, FunctionBuilder, HIRContext},
};

impl FunctionBuilder {
    pub fn build_static_access_expr(
        &mut self,
        ctx: &mut HIRContext,
        left: Box<Expr>,
        field: IdentifierNode,
    ) -> Value {
        todo!("Implement static access expression builder")
    }
}
