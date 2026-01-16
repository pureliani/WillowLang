use crate::{
    ast::expr::Expr,
    hir::{
        cfg::{Terminator, Value},
        errors::{SemanticError, SemanticErrorKind},
        types::checked_type::Type,
        utils::check_is_assignable::check_is_assignable,
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_or_expr(
        &mut self,
        ctx: &mut HIRContext,
        left: Box<Expr>,
        right: Box<Expr>,
    ) -> Value {
        let right_entry_block_id = self.new_basic_block();
        let merge_block_id = self.new_basic_block();

        let result_param = self.append_block_param(ctx, merge_block_id, Type::Bool);

        let left_span = left.span;
        let left_value = self.build_expr(ctx, *left);

        let left_type = ctx.program_builder.get_value_type(&left_value);
        if !check_is_assignable(&left_type, &Type::Bool) {
            return Value::Use(self.report_error_and_get_poison(
                ctx,
                SemanticError {
                    kind: SemanticErrorKind::TypeMismatch {
                        expected: Type::Bool,
                        received: left_type,
                    },
                    span: left_span,
                },
            ));
        }

        if let Value::Use(left_id) = left_value {
            if let Some(pred) = self.predicates.get(&left_id).cloned() {
                let local_f =
                    self.use_value_in_block(ctx, right_entry_block_id, pred.target_ptr);
                self.refinements
                    .insert((right_entry_block_id, local_f), pred.false_type);

                let local_t =
                    self.use_value_in_block(ctx, merge_block_id, pred.target_ptr);
                self.refinements
                    .insert((merge_block_id, local_t), pred.true_type);
            }
        }

        self.set_basic_block_terminator(Terminator::CondJump {
            condition: left_value,
            true_target: merge_block_id,
            true_args: vec![Value::BoolLiteral(true)],
            false_target: right_entry_block_id,
            false_args: vec![],
        });

        self.seal_block(ctx, right_entry_block_id);

        self.use_basic_block(right_entry_block_id);
        let right_span = right.span;
        let right_value = self.build_expr(ctx, *right);

        let right_type = ctx.program_builder.get_value_type(&right_value);
        if !check_is_assignable(&right_type, &Type::Bool) {
            return Value::Use(self.report_error_and_get_poison(
                ctx,
                SemanticError {
                    kind: SemanticErrorKind::TypeMismatch {
                        expected: Type::Bool,
                        received: right_type,
                    },
                    span: right_span,
                },
            ));
        }

        self.set_basic_block_terminator(Terminator::Jump {
            target: merge_block_id,
            args: vec![right_value],
        });

        self.seal_block(ctx, merge_block_id);

        self.use_basic_block(merge_block_id);

        Value::Use(result_param)
    }
}
