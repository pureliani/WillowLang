use crate::{
    ast::{expr::Expr, Span},
    cfg::{BinaryOperationKind, Instruction, Value},
    ensure,
    hir_builder::{
        errors::{SemanticError, SemanticErrorKind},
        types::checked_type::{Type, TypeKind},
        utils::check_is_equatable::check_is_equatable,
        HIRBuilder,
    },
};

impl<'a> HIRBuilder<'a> {
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
        let left_type = left_value.get_value_type(&self.cfg.value_types);

        let right_value = self.build_expr(*right);
        let right_type = right_value.get_value_type(&self.cfg.value_types);

        ensure!(
            self,
            check_is_equatable(&left_type.kind, &right_type.kind),
            SemanticError {
                kind: SemanticErrorKind::CannotCompareType {
                    of: left_type,
                    to: right_type
                },
                span
            }
        );

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
