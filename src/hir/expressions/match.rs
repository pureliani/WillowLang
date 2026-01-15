use crate::{
    ast::expr::{Expr, MatchArm},
    hir::{cfg::Value, FunctionBuilder, HIRContext},
};

impl FunctionBuilder {
    pub fn build_match_expr(
        &mut self,
        ctx: &mut HIRContext,
        conditions: Vec<Expr>,
        arms: Vec<MatchArm>,
    ) -> Value {
        todo!("Implement match expression builder")
    }
}
