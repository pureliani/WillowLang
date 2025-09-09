use crate::{
    ast::{expr::Expr, Span},
    cfg::{BinaryOperationKind, Instruction, Value},
    hir_builder::{
        errors::{SemanticError, SemanticErrorKind},
        types::checked_type::{Type, TypeKind},
        utils::check_is_equatable::check_is_equatable,
        FunctionBuilder,
    },
};

impl FunctionBuilder {
    pub fn build_equality_expr(&mut self, left: Box<Expr>, right: Box<Expr>, op_kind: BinaryOperationKind) -> Value {
        let span = Span {
            start: left.span.start,
            end: right.span.end,
        };

        let result_type = Type {
            kind: TypeKind::Bool,
            span,
        };

        let left_value = self.build_expr(*left);
        let left_type = self.get_value_type(&left_value);

        let right_value = self.build_expr(*right);
        let right_type = self.get_value_type(&right_value);

        if !check_is_equatable(&left_type.kind, &right_type.kind) {
            return self.report_error_and_get_poison(SemanticError {
                kind: SemanticErrorKind::CannotCompareType {
                    of: left_type,
                    to: right_type,
                },
                span,
            });
        }

        let destination_id = self.new_value_id();
        self.cfg.value_types.insert(destination_id, result_type.clone());

        self.add_basic_block_instruction(Instruction::BinaryOp {
            op_kind,
            destination: destination_id,
            left: left_value,
            right: right_value,
        });

        Value::Use(destination_id)
    }
}
