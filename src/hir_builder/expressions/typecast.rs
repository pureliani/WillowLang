use crate::{
    ast::{expr::Expr, type_annotation::TypeAnnotation},
    cfg::Value,
    hir_builder::{FunctionBuilder, HIRContext},
};

impl FunctionBuilder {
    pub fn build_typecast_expr(&mut self, ctx: &mut HIRContext, left: Box<Expr>, target: TypeAnnotation) -> Value {
        let value = self.build_expr(ctx, *left);
        let target_type = self.check_type_annotation(ctx, &target);
        Value::Use(self.emit_type_cast(ctx, value, target_type))
    }
}
