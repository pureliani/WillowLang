use crate::{
    ast::expr::Expr,
    hir::{
        cfg::{Terminator, Value},
        types::checked_type::Type,
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_and_expr(
        &mut self,
        ctx: &mut HIRContext,
        left: Box<Expr>,
        right: Box<Expr>,
    ) -> Value {
        let right_entry_block_id = self.new_basic_block();
        let merge_block_id = self.new_basic_block();

        let result_param = self.append_block_param(ctx, merge_block_id, Type::Bool);

        let left_value = self.build_expr(ctx, *left);

        self.set_basic_block_terminator(Terminator::CondJump {
            condition: left_value,
            true_target: right_entry_block_id,
            true_args: vec![],
            false_target: merge_block_id,
            false_args: vec![Value::BoolLiteral(false)],
        });

        self.seal_block(ctx, right_entry_block_id);

        self.use_basic_block(right_entry_block_id);
        let right_value = self.build_expr(ctx, *right);

        self.set_basic_block_terminator(Terminator::Jump {
            target: merge_block_id,
            args: vec![right_value],
        });

        self.seal_block(ctx, merge_block_id);

        self.use_basic_block(merge_block_id);

        Value::Use(result_param)
    }
}
