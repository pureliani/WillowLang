use crate::{
    ast::expr::Expr,
    hir::{
        cfg::{BinaryOperationKind, Value},
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_binary_op_expr(
        &mut self,
        ctx: &mut HIRContext,
        left: Box<Expr>,
        right: Box<Expr>,
        op_kind: BinaryOperationKind,
    ) -> Value {
        let left_span = left.span;
        let right_span = right.span;
        let left_value = self.build_expr(ctx, *left);
        let right_value = self.build_expr(ctx, *right);
        let destination = match self.emit_binary_op(
            ctx,
            op_kind,
            left_value,
            left_span,
            right_value,
            right_span,
        ) {
            Ok(destination_id) => destination_id,
            Err(error) => {
                return Value::Use(self.report_error_and_get_poison(ctx, error));
            }
        };

        Value::Use(destination)
    }
}
