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

        let (union_ptr_id, union_type) = match self.build_lvalue_expr(ctx, *left.clone())
        {
            Ok(ptr) => {
                let ptr_ty = self.get_refined_type(ctx, self.current_block_id, ptr);
                match ptr_ty {
                    Type::Pointer(inner) => (ptr, *inner),
                    _ => panic!("INTERNAL COMPILER ERROR: L-value must be a pointer"),
                }
            }
            Err(_) => {
                // store in a temporary stack slot.
                let val = self.build_expr(ctx, *left);
                let ty = ctx.program_builder.get_value_type(&val);
                let tmp_ptr = self.emit_stack_alloc(ctx, ty.clone(), 1);
                self.emit_store(ctx, tmp_ptr, val);
                (tmp_ptr, ty)
            }
        };

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

        let base_refined_ptr_ty =
            self.get_refined_type(ctx, self.current_block_id, union_ptr_id);

        let id_field = IdentifierNode {
            name: ctx.program_builder.common_identifiers.id,
            span: left_span,
        };
        let id_ptr = match self.emit_get_field_ptr(ctx, union_ptr_id, id_field) {
            Ok(ptr) => ptr,
            Err(e) => return Value::Use(self.report_error_and_get_poison(ctx, e)),
        };
        let actual_id = Value::Use(self.emit_load(ctx, id_ptr));

        let true_path = self.new_basic_block();
        let false_path = self.new_basic_block();
        let merge_block = self.new_basic_block();
        let mut target_tag_ids = Vec::new();

        for (i, tag_ann) in variants.iter().enumerate() {
            if tag_ann.value_type.is_some() {
                let err = SemanticError {
                    kind: SemanticErrorKind::ValuedTagInIsExpression,
                    span: tag_ann.span,
                };
                return Value::Use(self.report_error_and_get_poison(ctx, err));
            }

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

        let local_union_ptr_id = self.use_value_in_block(ctx, true_path, union_ptr_id);
        let narrowed_ptr_ty = intersect_types(&base_refined_ptr_ty, &target_tag_ids);
        self.refinements
            .insert((true_path, local_union_ptr_id), narrowed_ptr_ty);

        self.set_basic_block_terminator(Terminator::Jump {
            target: merge_block,
            args: vec![Value::BoolLiteral(true)],
        });

        self.seal_block(ctx, false_path);
        self.use_basic_block(false_path);

        let local_union_ptr_id_f = self.use_value_in_block(ctx, false_path, union_ptr_id);
        let remainder_ptr_ty = subtract_types(&base_refined_ptr_ty, &target_tag_ids);
        self.refinements
            .insert((false_path, local_union_ptr_id_f), remainder_ptr_ty);

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
