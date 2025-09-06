use crate::{
    ast::expr::Expr,
    cfg::{BinaryOperationKind, Instruction, Value},
    hir_builder::HIRBuilder,
};

impl<'a> HIRBuilder<'a> {
    pub fn build_arithmetic_expr(&mut self, left: Box<Expr>, right: Box<Expr>, op_kind: BinaryOperationKind) -> Value {
        let left_value = self.build_expr(*left);
        let left_type = left_value.get_value_type(&self.cfg.value_types);

        let right_value = self.build_expr(*right);
        let right_type = right_value.get_value_type(&self.cfg.value_types);

        let validation_result = self.check_binary_numeric_operation(&left_type, &right_type);

        let result_type = match validation_result {
            Ok(t) => t,
            Err(e) => return self.report_error_and_get_poison(e),
        };

        let destination = self.new_value_id();
        self.cfg.value_types.insert(destination, result_type);

        self.add_basic_block_instruction(Instruction::BinaryOp {
            op_kind,
            destination,
            left: left_value,
            right: right_value,
        });

        Value::Use(destination)
    }
}
