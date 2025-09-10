use crate::{
    ast::expr::Expr,
    cfg::{BinaryOperationKind, Instruction, Value},
    hir_builder::{FunctionBuilder, ModuleBuilder},
};

impl FunctionBuilder {
    pub fn build_arithmetic_expr(
        &mut self,
        module_builder: &mut ModuleBuilder,
        left: Box<Expr>,
        right: Box<Expr>,
        op_kind: BinaryOperationKind,
    ) -> Value {
        let left_value = self.build_expr(module_builder, *left);
        let left_type = self.get_value_type(&left_value);

        let right_value = self.build_expr(module_builder, *right);
        let right_type = self.get_value_type(&right_value);

        let validation_result = self.check_binary_numeric_operation(&left_type, &right_type);

        let result_type = match validation_result {
            Ok(t) => t,
            Err(e) => return self.report_error_and_get_poison(module_builder, e),
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
