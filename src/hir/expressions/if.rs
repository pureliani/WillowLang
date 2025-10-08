use crate::{
    ast::expr::{BlockContents, Expr},
    hir::{
        cfg::{BasicBlockId, Terminator, Value},
        errors::{SemanticError, SemanticErrorKind},
        types::checked_type::{Type, TypeKind},
        FunctionBuilder, HIRContext,
    },
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IfContext {
    /// The `if` is used to produce a value
    Expression,
    /// The `if` is used for control flow, its value is discarded
    Statement,
}

impl FunctionBuilder {
    pub fn build_if(
        &mut self,
        ctx: &mut HIRContext,
        branches: Vec<(Box<Expr>, BlockContents)>,
        else_branch: Option<BlockContents>,
        context: IfContext,
    ) -> Value {
        if context == IfContext::Expression && else_branch.is_none() {
            let span = branches.first().unwrap().0.span;
            return Value::Use(self.report_error_and_get_poison(
                ctx,
                SemanticError {
                    kind: SemanticErrorKind::IfExpressionMissingElse,
                    span,
                },
            ));
        }

        let merge_block_id = self.new_basic_block();
        let mut last_condition_block_id = self.current_block_id;

        let mut phi_sources: Vec<(BasicBlockId, Value)> = Vec::new();

        for (condition, body) in branches {
            self.use_basic_block(last_condition_block_id);

            let condition_value = self.build_expr(ctx, *condition);
            let condition_value_type =
                ctx.program_builder.get_value_type(&condition_value);
            let expected_condition_type = Type {
                kind: TypeKind::Bool,
                span: condition_value_type.span,
            };

            if !self.check_is_assignable(&condition_value_type, &expected_condition_type)
            {
                return Value::Use(self.report_error_and_get_poison(
                    ctx,
                    SemanticError {
                        span: condition_value_type.span,
                        kind: SemanticErrorKind::TypeMismatch {
                            expected: expected_condition_type,
                            received: condition_value_type,
                        },
                    },
                ));
            }

            let body_block_id = self.new_basic_block();
            let next_condition_block_id = self.new_basic_block();

            self.set_basic_block_terminator(Terminator::CondJump {
                condition: condition_value,
                true_target: body_block_id,
                false_target: next_condition_block_id,
            });

            self.use_basic_block(body_block_id);
            let body_value = self.build_codeblock_expr(ctx, body);
            let body_exit_block_id = self.current_block_id;
            phi_sources.push((body_exit_block_id, body_value));
            self.set_basic_block_terminator(Terminator::Jump {
                target: merge_block_id,
            });

            last_condition_block_id = next_condition_block_id;
        }

        self.use_basic_block(last_condition_block_id);

        if let Some(else_body) = else_branch {
            let else_value = self.build_codeblock_expr(ctx, else_body);
            let else_exit_block_id = self.current_block_id;
            phi_sources.push((else_exit_block_id, else_value));
        }

        self.set_basic_block_terminator(Terminator::Jump {
            target: merge_block_id,
        });

        self.use_basic_block(merge_block_id);

        if context == IfContext::Expression {
            let phi_destination = match self.emit_phi(ctx, phi_sources) {
                Ok(id) => id,
                Err(e) => return Value::Use(self.report_error_and_get_poison(ctx, e)),
            };
            Value::Use(phi_destination)
        } else {
            Value::VoidLiteral
        }
    }
}
