use crate::{
    ast::{expr::Expr, Span},
    cfg::{BinaryOperationKind, Instruction, Value},
    hir_builder::{
        types::checked_type::{Type, TypeKind},
        HIRBuilder,
    },
};

impl<'a> HIRBuilder<'a> {
    pub fn build_comparison_expr(&mut self, left: Box<Expr>, right: Box<Expr>, op_kind: BinaryOperationKind) -> Value {
        let result_type = Type {
            kind: TypeKind::Bool,
            span: Span {
                start: left.span.start,
                end: right.span.end,
            },
        };

        let left_value = self.build_expr(*left);
        let left_type = left_value.get_value_type(&self.cfg.value_types);

        let right_value = self.build_expr(*right);
        let right_type = right_value.get_value_type(&self.cfg.value_types);

        let validation_result = self.check_binary_numeric_operation(&left_type, &right_type);

        if let Err(e) = validation_result {
            return self.report_error_and_get_poison(e);
        };

        let destination = self.new_value_id();
        self.cfg.value_types.insert(destination, result_type.clone());
        self.add_basic_block_instruction(Instruction::BinaryOp {
            op_kind,
            destination,
            left: left_value,
            right: right_value,
        });

        Value::Use(destination)
    }
}
