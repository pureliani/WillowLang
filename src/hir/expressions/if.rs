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
    /// The `if` is used to produce a value.
    Expression,
    /// The `if` is used for control flow; its value is discarded.
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

            let condition_value = self.build_expr(ctx, *condition);
            let condition_value_type =
                ctx.program_builder.get_value_type(&condition_value);

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
            let body_type = ctx.program_builder.get_value_type(&body_value);
            let body_exit_block_id = self.current_block_id;
            phi_sources.push((body_exit_block_id, body_value, body_type));
            self.set_basic_block_terminator(Terminator::Jump {
                target: merge_block_id,
            });

            current_block_id = next_condition_block_id;
        }

        self.use_basic_block(current_block_id);
        if let Some(else_body) = else_branch {
            let else_value = self.build_codeblock_expr(ctx, else_body);
            let else_type = ctx.program_builder.get_value_type(&else_value);
            let else_exit_block_id = self.current_block_id;
            phi_sources.push((else_exit_block_id, else_value, else_type));
        }
        self.set_basic_block_terminator(Terminator::Jump {
            target: merge_block_id,
        });

        self.use_basic_block(merge_block_id);

        if context == IfContext::Expression {
            let sources_for_phi: Vec<(BasicBlockId, Value)> = phi_sources
                .into_iter()
                .map(|(id, val, _)| (id, val))
                .collect();

            let phi_destination = match self.emit_phi(ctx, sources_for_phi) {
                Ok(id) => id,
                Err(e) => return Value::Use(self.report_error_and_get_poison(ctx, e)),
            };

            Value::Use(phi_destination)
        } else {
            Value::VoidLiteral
        }
    }
}
