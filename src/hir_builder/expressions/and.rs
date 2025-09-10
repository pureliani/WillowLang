use crate::{
    ast::expr::Expr,
    cfg::{Instruction, Terminator, Value},
    hir_builder::{FunctionBuilder, ModuleBuilder},
};

impl FunctionBuilder {
    pub fn build_and_expr(&mut self, module_builder: &mut ModuleBuilder, left: Box<Expr>, right: Box<Expr>) -> Value {
        let right_entry_block_id = self.new_basic_block();
        let merge_block_id = self.new_basic_block();

        let left_value = self.build_expr(module_builder, *left);
        let left_exit_block_id = self.current_block_id;

        self.set_basic_block_terminator(Terminator::CondJump {
            condition: left_value,
            true_target: right_entry_block_id,
            false_target: merge_block_id,
        });

        self.use_basic_block(right_entry_block_id);
        let right_value = self.build_expr(module_builder, *right);
        let right_exit_block_id = self.current_block_id;
        self.set_basic_block_terminator(Terminator::Jump { target: merge_block_id });

        self.use_basic_block(merge_block_id);
        let phi_destination = self.new_value_id();
        let phi_instruction = Instruction::Phi {
            destination: phi_destination,
            sources: vec![
                (left_exit_block_id, Value::BoolLiteral(false)),
                (right_exit_block_id, right_value),
            ],
        };
        self.add_basic_block_instruction(phi_instruction);

        Value::Use(phi_destination)
    }
}
