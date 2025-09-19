use crate::{
    ast::expr::{Expr, MatchArm},
    cfg::Value,
    hir_builder::{FunctionBuilder, HIRContext},
};

impl FunctionBuilder {
    pub fn build_match_expr(&mut self, ctx: &mut HIRContext, conditions: Vec<Expr>, arms: Vec<MatchArm>) -> Value {
        todo!()
    }
}
