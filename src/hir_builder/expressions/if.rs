use crate::{
    ast::expr::{BlockContents, Expr},
    cfg::{BasicBlockId, Instruction, Terminator, Value},
    hir_builder::{
        errors::{SemanticError, SemanticErrorKind},
        types::checked_type::{Type, TypeKind},
        HIRBuilder,
    },
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IfContext {
    /// The `if` is used to produce a value.
    Expression,
    /// The `if` is used for control flow; its value is discarded.
    Statement,
}

impl<'a> HIRBuilder<'a> {
    pub fn build_if(
        &mut self,
        branches: Vec<(Box<Expr>, BlockContents)>,
        else_branch: Option<BlockContents>,
        context: IfContext,
    ) -> Value {
        if context == IfContext::Expression && else_branch.is_none() {
            let span = branches.first().unwrap().0.span;
            return self.report_error_and_get_poison(SemanticError {
                kind: SemanticErrorKind::IfExpressionMissingElse,
                span,
            });
        }

        let entry_block_id = self.current_block_id;
        let merge_block_id = self.new_basic_block();

        let mut phi_sources: Vec<(BasicBlockId, Value, Type)> = Vec::new();
        let mut current_block_id = entry_block_id;

        for (condition, body) in branches {
            let expected_condition_type = Type {
                kind: TypeKind::Bool,
                span: condition.span,
            };

            self.use_basic_block(current_block_id);

            let condition_value = self.build_expr(*condition);
            let condition_value_type = self.get_value_type(&condition_value);

            if !self.check_is_assignable(&condition_value_type, &expected_condition_type) {
                return self.report_error_and_get_poison(SemanticError {
                    span: condition_value_type.span,
                    kind: SemanticErrorKind::TypeMismatch {
                        expected: expected_condition_type,
                        received: condition_value_type,
                    },
                });
            }

            let body_block_id = self.new_basic_block();
            let next_condition_block_id = self.new_basic_block();

            self.set_basic_block_terminator(Terminator::CondJump {
                condition: condition_value,
                true_target: body_block_id,
                false_target: next_condition_block_id,
            });

            self.use_basic_block(body_block_id);
            let body_value = self.build_codeblock_expr(body);
            let body_type = self.get_value_type(&body_value);
            let body_exit_block_id = self.current_block_id;
            phi_sources.push((body_exit_block_id, body_value, body_type));
            self.set_basic_block_terminator(Terminator::Jump { target: merge_block_id });

            current_block_id = next_condition_block_id;
        }

        self.use_basic_block(current_block_id);
        if let Some(else_body) = else_branch {
            let else_value = self.build_codeblock_expr(else_body);
            let else_type = self.get_value_type(&else_value);
            let else_exit_block_id = self.current_block_id;
            phi_sources.push((else_exit_block_id, else_value, else_type));
        }
        self.set_basic_block_terminator(Terminator::Jump { target: merge_block_id });

        self.use_basic_block(merge_block_id);

        if context == IfContext::Expression {
            let first_branch_type = &phi_sources
            .first()
            .expect("INTERNAL COMPILER ERROR: `build_if` was called in Expression context with no branches. This should be prevented by the parser.")
            .2;

            for branch_type in phi_sources.iter().map(|phi_source| &phi_source.2) {
                if !self.check_is_assignable(branch_type, first_branch_type) {
                    return self.report_error_and_get_poison(SemanticError {
                        kind: SemanticErrorKind::IncompatibleBranchTypes {
                            first: first_branch_type.clone(),
                            second: branch_type.clone(),
                        },
                        span: branch_type.span,
                    });
                }
            }

            let phi_destination = self.new_value_id();
            self.cfg.value_types.insert(phi_destination, first_branch_type.clone());

            let sources_for_phi = phi_sources.into_iter().map(|(id, val, _)| (id, val)).collect();
            self.add_basic_block_instruction(Instruction::Phi {
                destination: phi_destination,
                sources: sources_for_phi,
            });

            Value::Use(phi_destination)
        } else {
            Value::VoidLiteral
        }
    }
}
