use crate::{
    ast::{expr::Expr, type_annotation::TypeAnnotation},
    hir::{
        cfg::Value, utils::check_type::check_type_annotation, FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_typecast_expr(
        &mut self,
        ctx: &mut HIRContext,
        left: Box<Expr>,
        target: TypeAnnotation,
    ) -> Value {
        let value_span = left.span;
        let value = self.build_expr(ctx, *left);
        let target_type = check_type_annotation(ctx, &target);
        Value::Use(self.emit_type_cast(ctx, value, value_span, target_type))
    }
}
