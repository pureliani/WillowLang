use crate::{
    ast::expr::Expr,
    hir::{
        cfg::Value,
        types::checked_type::{PointerKind, Type},
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_ptr_expr(
        &mut self,
        ctx: &mut HIRContext,
        inner: Box<Expr>,
        is_mutable: bool,
    ) -> Value {
        let lvalue_result = self.build_lvalue_expr(ctx, *inner);

        match lvalue_result {
            Ok(ptr_id) => {
                let inner_type = match ctx.program_builder.get_value_id_type(&ptr_id) {
                    Type::Pointer { to, .. } => *to,
                    t => t,
                };

                let target_kind = if is_mutable {
                    PointerKind::Mut
                } else {
                    PointerKind::Ref
                };

                let target_type = Type::Pointer {
                    kind: target_kind,
                    to: Box::new(inner_type),
                };

                Value::Use(self.emit_type_cast(ctx, Value::Use(ptr_id), target_type))
            }
            Err(e) => Value::Use(self.report_error_and_get_poison(ctx, e)),
        }
    }
}
