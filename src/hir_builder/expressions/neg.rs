use crate::{
    ast::expr::Expr,
    cfg::{UnaryOperationKind, Value},
    hir_builder::{FunctionBuilder, HIRContext},
};

impl FunctionBuilder {
    pub fn build_airthmetic_negation_expr(&mut self, ctx: &mut HIRContext, expr: Box<Expr>) -> Value {
        let value = self.build_expr(ctx, *expr);
        let destination = self.emit_unary_op(ctx, UnaryOperationKind::Neg, value);
        Value::Use(destination)
    }
}
