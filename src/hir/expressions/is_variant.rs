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

        let id_field = IdentifierNode {
            name: ctx.program_builder.common_identifiers.id,
            span: left_span,
        };
        let id_ptr = match self.emit_get_field_ptr(ctx, union_id, id_field) {
            Ok(ptr) => ptr,
            Err(e) => return Value::Use(self.report_error_and_get_poison(ctx, e)),
        };
        let actual_id = Value::Use(self.emit_load(ctx, id_ptr));

        let true_path = self.new_basic_block();
        let false_path = self.new_basic_block();
        let merge_block = self.new_basic_block();

        let mut target_tag_ids = Vec::new();

        // For each variant, if it matches, jump to true_path.
        // If it doesn't, jump to the next check (or false_path if it's the last one).
        for (i, tag_ann) in variants.iter().enumerate() {
            let tag_id = ctx
                .program_builder
                .tag_interner
                .intern(&tag_ann.identifier.name);
            target_tag_ids.push(tag_id);

            let is_match = match self.emit_binary_op(
                ctx,
                BinaryOperationKind::Equal,
                actual_id.clone(),
                left_span,
                Value::NumberLiteral(NumberKind::U16(tag_id.0)),
                tag_ann.span,
            ) {
                Ok(id) => id,
                Err(e) => return Value::Use(self.report_error_and_get_poison(ctx, e)),
            };

            let is_last = i == variants.len() - 1;
            let next_check_block = if is_last {
                false_path
            } else {
                self.new_basic_block()
            };

            self.set_basic_block_terminator(Terminator::CondJump {
                condition: Value::Use(is_match),
                true_target: true_path,
                true_args: vec![],
                false_target: next_check_block,
                false_args: vec![],
            });

            if !is_last {
                self.seal_block(ctx, next_check_block);
                self.use_basic_block(next_check_block);
            }
        }

        self.seal_block(ctx, true_path);
        self.use_basic_block(true_path);

        let current_ty = self.get_refined_type(ctx, true_path, union_id);
        let narrowed_ty = intersect_types(&current_ty, &target_tag_ids);
        self.refinements.insert((true_path, union_id), narrowed_ty);

        self.set_basic_block_terminator(Terminator::Jump {
            target: merge_block,
            args: vec![Value::BoolLiteral(true)],
        });

        self.seal_block(ctx, false_path);
        self.use_basic_block(false_path);

        let remainder_ty = subtract_types(&current_ty, &target_tag_ids);
        self.refinements
            .insert((false_path, union_id), remainder_ty);

        self.set_basic_block_terminator(Terminator::Jump {
            target: merge_block,
            args: vec![Value::BoolLiteral(false)],
        });

        self.seal_block(ctx, merge_block);
        self.use_basic_block(merge_block);
        let result_bool = self.append_block_param(ctx, merge_block, Type::Bool);

        Value::Use(result_bool)
    }
}
