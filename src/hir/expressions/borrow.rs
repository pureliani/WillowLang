use crate::{
    ast::expr::{BorrowKind, Expr},
    hir::{
        cfg::Value,
        errors::{SemanticError, SemanticErrorKind},
        types::checked_type::{PointerKind, Type, TypeKind},
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_borrow_expr(&mut self, ctx: &mut HIRContext, kind: BorrowKind, value: Box<Expr>) -> Value {
        let value_span = value.span;
        let lvalue_ptr_id = match self.build_lvalue_expr(ctx, *value) {
            Ok(ptr_value_id) => ptr_value_id,
            Err(error) => self.report_error_and_get_poison(ctx, error),
        };

        let lvalue_ptr_type = ctx.program_builder.get_value_id_type(&lvalue_ptr_id);

        if let TypeKind::Pointer {
            kind: PointerKind::Raw,
            value_type: target_type,
        } = lvalue_ptr_type.kind
        {
            let borrow_pointer_kind = match kind {
                BorrowKind::Shared => PointerKind::SharedBorrow,
                BorrowKind::Mutable => PointerKind::MutableBorrow,
            };

            let target_type = Type {
                span: value_span,
                kind: TypeKind::Pointer {
                    kind: borrow_pointer_kind,
                    value_type: target_type,
                },
            };

            let cast_value_id = self.emit_type_cast(ctx, Value::Use(lvalue_ptr_id), target_type);

            return Value::Use(cast_value_id);
        } else {
            Value::Use(self.report_error_and_get_poison(
                ctx,
                SemanticError {
                    kind: SemanticErrorKind::CannotBorrow(lvalue_ptr_type),
                    span: value_span,
                },
            ))
        }
    }
}
