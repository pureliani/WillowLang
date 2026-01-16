use crate::{
    ast::expr::Expr,
    hir::{
        cfg::{UnaryOperationKind, Value},
        FunctionBuilder, HIRContext, TypePredicate,
    },
};

impl FunctionBuilder {
    pub fn build_unary_op_expr(
        &mut self,
        ctx: &mut HIRContext,
        op_kind: UnaryOperationKind,
        expr: Box<Expr>,
    ) -> Value {
        let value = self.build_expr(ctx, *expr);
        let destination = match self.emit_unary_op(ctx, op_kind.clone(), value.clone()) {
            Ok(destination_id) => destination_id,
            Err(error) => {
                return Value::Use(self.report_error_and_get_poison(ctx, error));
            }
        };

        if matches!(op_kind, UnaryOperationKind::Not) {
            if let Value::Use(operand_id) = value {
                if let Some(pred) = self.predicates.get(&operand_id).cloned() {
                    self.predicates.insert(
                        destination,
                        TypePredicate {
                            source: pred.source,
                            true_id: pred.false_id,
                            false_id: pred.true_id,
                        },
                    );
                }
            }
        }

        Value::Use(destination)
    }
}
