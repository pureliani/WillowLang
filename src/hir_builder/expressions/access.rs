use crate::{
    ast::{expr::Expr, IdentifierNode},
    cfg::Value,
    hir_builder::{FunctionBuilder, HIRContext},
};

impl FunctionBuilder {
    pub fn build_access_expr(&mut self, ctx: &mut HIRContext, left: Box<Expr>, field: IdentifierNode) -> Value {
        let base_ptr_id = match self.build_lvalue_expr(ctx, *left) {
            Ok(id) => id,
            Err(e) => return self.report_error_and_get_poison(ctx, e),
        };

        let field_ptr_id = match self.emit_get_field_ptr(base_ptr_id, field) {
            Ok(id) => id,
            Err(e) => return self.report_error_and_get_poison(ctx, e),
        };

        let final_value_id = self.emit_load(field_ptr_id);

        Value::Use(final_value_id)
    }
}
