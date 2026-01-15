use crate::{
    ast::{expr::Expr, type_annotation::TagAnnotation, IdentifierNode},
    hir::{
        cfg::{BinaryOperationKind, Terminator, Value},
        errors::{SemanticError, SemanticErrorKind},
        types::checked_type::{StructKind, Type},
        utils::try_unify_types::{intersect_types, subtract_types},
        FunctionBuilder, HIRContext,
    },
    tokenize::NumberKind,
};

impl FunctionBuilder {
    pub fn build_is_variant_expr(
        &mut self,
        ctx: &mut HIRContext,
        left: Box<Expr>,
        variants: Vec<TagAnnotation>,
    ) -> Value {
        let left_span = left.span;
        let union_val = self.build_expr(ctx, *left);
        let union_type = ctx.program_builder.get_value_type(&union_val);

        if !matches!(
            union_type,
            Type::Struct(StructKind::Union { .. }) | Type::Struct(StructKind::Tag(_))
        ) {
            let err = SemanticError {
                kind: SemanticErrorKind::CannotAccess(union_type),
                span: left_span,
            };
            return Value::Use(self.report_error_and_get_poison(ctx, err));
        }

        let union_id = match union_val {
            Value::Use(id) => id,
            _ => panic!(
                "INTERNAL COMPILER ERROR: Expected Value::Use for narrowing target"
            ),
        };

        let true_path = self.new_basic_block();
        let false_path = self.new_basic_block();
        let merge_block = self.new_basic_block();

        let id_field = IdentifierNode {
            name: ctx.program_builder.common_identifiers.id,
            span: left_span,
        };
        let id_ptr = match self.emit_get_field_ptr(ctx, union_id, id_field) {
            Ok(ptr) => ptr,
            Err(e) => return Value::Use(self.report_error_and_get_poison(ctx, e)),
        };
        let actual_id = Value::Use(self.emit_load(ctx, id_ptr));

        todo!()
    }
}
