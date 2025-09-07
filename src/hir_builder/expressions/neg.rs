use std::collections::HashSet;

use crate::{
    ast::expr::Expr,
    cfg::{Instruction, UnaryOperationKind, Value},
    hir_builder::{
        errors::{SemanticError, SemanticErrorKind},
        types::checked_type::{Type, TypeKind},
        utils::is_signed::is_signed,
        HIRBuilder,
    },
};

impl<'a> HIRBuilder<'a> {
    pub fn build_airthmetic_negation_expr(&mut self, expr: Box<Expr>) -> Value {
        let span = expr.span;
        let value = self.build_expr(*expr);
        let value_type = self.get_value_type(&value);

        if !is_signed(&value_type.kind) {
            let expected = HashSet::from([
                Type {
                    kind: TypeKind::I8,
                    span,
                },
                Type {
                    kind: TypeKind::I16,
                    span,
                },
                Type {
                    kind: TypeKind::I32,
                    span,
                },
                Type {
                    kind: TypeKind::I64,
                    span,
                },
                Type {
                    kind: TypeKind::ISize,
                    span,
                },
                Type {
                    kind: TypeKind::F32,
                    span,
                },
                Type {
                    kind: TypeKind::F64,
                    span,
                },
            ]);

            return self.report_error_and_get_poison(SemanticError {
                kind: SemanticErrorKind::TypeMismatchExpectedOneOf {
                    expected,
                    received: value_type.clone(),
                },
                span,
            });
        }

        let destination = self.new_value_id();
        self.cfg.value_types.insert(destination, value_type);
        self.add_basic_block_instruction(Instruction::UnaryOp {
            op_kind: UnaryOperationKind::Neg,
            destination,
            operand: value,
        });

        Value::Use(destination)
    }
}
