use crate::{
    ast::expr::Expr,
    hir::{
        cfg::{Terminator, Value},
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_and_expr(&mut self, ctx: &mut HIRContext, left: Box<Expr>, right: Box<Expr>) -> Value {
        let right_entry_block_id = self.new_basic_block();
        let merge_block_id = self.new_basic_block();

        let left_value = self.build_expr(ctx, *left);
        let left_exit_block_id = self.current_block_id;

        self.set_basic_block_terminator(Terminator::CondJump {
            condition: left_value,
            true_target: right_entry_block_id,
            false_target: merge_block_id,
        });

        self.use_basic_block(right_entry_block_id);
        let right_value = self.build_expr(ctx, *right);
        let right_exit_block_id = self.current_block_id;
        self.set_basic_block_terminator(Terminator::Jump { target: merge_block_id });

        self.use_basic_block(merge_block_id);
        let phi_sources = vec![
            (left_exit_block_id, Value::BoolLiteral(false)),
            (right_exit_block_id, right_value),
        ];

        let phi_destination = match self.emit_phi(phi_sources) {
            Ok(phi_destination) => phi_destination,
            Err(err) => self.report_error_and_get_poison(ctx, err),
        };

        Value::Use(phi_destination)
    }
}
