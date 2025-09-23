use crate::{
    ast::expr::Expr,
    hir::{
        cfg::{UnaryOperationKind, Value},
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_unary_op_expr(&mut self, ctx: &mut HIRContext, op_kind: UnaryOperationKind, expr: Box<Expr>) -> Value {
        let value = self.build_expr(ctx, *expr);
        let destination = match self.emit_unary_op(ctx, op_kind, value) {
            Ok(destination_id) => destination_id,
            Err(error) => {
                return Value::Use(self.report_error_and_get_poison(ctx, error));
            }
        };

        Value::Use(destination)
    }
}
