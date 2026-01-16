use crate::{
    ast::{
        expr::{BlockContents, Expr},
        Span,
    },
    hir::{
        cfg::{BasicBlockId, Terminator, Value},
        errors::{SemanticError, SemanticErrorKind},
        types::checked_type::Type,
        utils::{
            check_is_assignable::check_is_assignable, try_unify_types::try_unify_types,
        },
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
// src/hir/expressions/if.rs

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
        let mut branch_results: Vec<(BasicBlockId, Value, Span)> = Vec::new();
        let mut last_condition_block_id = self.current_block_id;

        for (condition, body) in branches {
            let condition_span = condition.span;
            let body_span = body.span;

            self.use_basic_block(last_condition_block_id);

            let condition_value = self.build_expr(ctx, *condition);
            let condition_value_type =
                ctx.program_builder.get_value_type(&condition_value);

            if !check_is_assignable(&condition_value_type, &Type::Bool) {
                ctx.module_builder.errors.push(SemanticError {
                    span: condition_span,
                    kind: SemanticErrorKind::TypeMismatch {
                        expected: Type::Bool,
                        received: condition_value_type,
                    },
                });
            }

            let body_block_id = self.new_basic_block();
            let next_condition_block_id = self.new_basic_block();

            if let Value::Use(cond_id) = condition_value {
                if let Some(pred) = self.predicates.get(&cond_id).cloned() {
                    let local_t =
                        self.use_value_in_block(ctx, body_block_id, pred.target_ptr);
                    self.refinements
                        .insert((body_block_id, local_t), pred.true_type);

                    let local_f = self.use_value_in_block(
                        ctx,
                        next_condition_block_id,
                        pred.target_ptr,
                    );
                    self.refinements
                        .insert((next_condition_block_id, local_f), pred.false_type);
                }
            }

            self.set_basic_block_terminator(Terminator::CondJump {
                condition: condition_value,
                true_target: body_block_id,
                true_args: vec![],
                false_target: next_condition_block_id,
                false_args: vec![],
            });

            self.seal_block(ctx, body_block_id);

            self.use_basic_block(body_block_id);
            let body_value = self.build_codeblock_expr(ctx, body);

            if self.get_current_basic_block().terminator.is_none() {
                branch_results.push((self.current_block_id, body_value, body_span));
            }

            self.seal_block(ctx, next_condition_block_id);
            last_condition_block_id = next_condition_block_id;
        }

        self.use_basic_block(last_condition_block_id);

        if let Some(else_body) = else_branch {
            let else_span = else_body.span;
            let else_value = self.build_codeblock_expr(ctx, else_body);

            if self.get_current_basic_block().terminator.is_none() {
                branch_results.push((self.current_block_id, else_value, else_span));
            }
        } else if context == IfContext::Statement {
            self.set_basic_block_terminator(Terminator::Jump {
                target: merge_block_id,
                args: vec![],
            });
        }

        let result_param_id = if context == IfContext::Expression {
            let type_entries: Vec<(Type, Span)> = branch_results
                .iter()
                .map(|(_, val, span)| (ctx.program_builder.get_value_type(val), *span))
                .collect();

            let result_type = match try_unify_types(&type_entries) {
                Ok(ty) => ty,
                Err(e) => {
                    ctx.module_builder.errors.push(e);
                    Type::Unknown
                }
            };

            Some(self.append_block_param(ctx, merge_block_id, result_type))
        } else {
            None
        };

        for (block_id, val, _) in branch_results {
            self.use_basic_block(block_id);
            let args = if result_param_id.is_some() {
                vec![val]
            } else {
                vec![]
            };

            self.set_basic_block_terminator(Terminator::Jump {
                target: merge_block_id,
                args,
            });
        }

        self.seal_block(ctx, merge_block_id);
        self.use_basic_block(merge_block_id);

        if let Some(pid) = result_param_id {
            Value::Use(pid)
        } else {
            Value::VoidLiteral
        }
    }
}
