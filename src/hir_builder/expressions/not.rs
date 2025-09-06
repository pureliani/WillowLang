use crate::{
    ast::expr::Expr,
    cfg::{Instruction, UnaryOperationKind, Value},
    hir_builder::{
        errors::{SemanticError, SemanticErrorKind},
        types::checked_type::{Type, TypeKind},
        HIRBuilder,
    },
};

impl<'a> HIRBuilder<'a> {
    pub fn build_not_expr(&mut self, expr: Box<Expr>) -> Value {
        let span = expr.span;

        let bool_type = Type {
            kind: TypeKind::Bool,
            span,
        };

        let value = self.build_expr(*expr);
        let value_type = value.get_value_type(&self.cfg.value_types);

        if !self.check_is_assignable(&value_type, &bool_type) {
            return self.report_error_and_get_poison(SemanticError {
                kind: SemanticErrorKind::TypeMismatch {
                    expected: bool_type.clone(),
                    received: value_type,
                },
                span,
            });
        }

        let result_id = self.new_value_id();
        self.cfg.value_types.insert(result_id, bool_type);
        self.add_basic_block_instruction(Instruction::UnaryOp {
            op_kind: UnaryOperationKind::Not,
            destination: result_id,
            operand: value,
        });

        Value::Use(result_id)
    }
}
