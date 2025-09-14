use crate::{
    ast::expr::Expr,
    cfg::{UnaryOperationKind, Value},
    hir_builder::{FunctionBuilder, HIRContext},
};

impl FunctionBuilder {
    pub fn build_not_expr(&mut self, ctx: &mut HIRContext, expr: Box<Expr>) -> Value {
        let value = self.build_expr(ctx, *expr);
        let result_id = self.emit_unary_op(ctx, UnaryOperationKind::Not, value);
        Value::Use(result_id)
    }
}
