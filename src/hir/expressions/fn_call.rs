use crate::{
    ast::{expr::Expr, Span},
    hir::{cfg::Value, FunctionBuilder, HIRContext},
};

impl FunctionBuilder {
    pub fn build_fn_call_expr(&mut self, ctx: &mut HIRContext, left: Box<Expr>, args: Vec<Expr>, span: Span) -> Value {
        let function_value = self.build_expr(ctx, *left);

        let arg_values: Vec<Value> = args.into_iter().map(|arg_expr| self.build_expr(ctx, arg_expr)).collect();

        match self.emit_function_call(ctx, function_value, arg_values, span) {
            Ok(Some(return_value_id)) => Value::Use(return_value_id),
            Ok(None) => Value::VoidLiteral,
            Err(e) => Value::Use(self.report_error_and_get_poison(ctx, e)),
        }
    }
}
