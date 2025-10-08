use crate::{
    ast::expr::{BlockContents, Expr},
    hir::{
        cfg::Terminator,
        errors::{SemanticError, SemanticErrorKind},
        types::checked_type::{Type, TypeKind},
        utils::scope::ScopeKind,
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_while_stmt(
        &mut self,
        ctx: &mut HIRContext,
        condition: Box<Expr>,
        body: BlockContents,
    ) {
        let condition_block = self.new_basic_block();
        let body_block = self.new_basic_block();
        let merge_block = self.new_basic_block();

        self.set_basic_block_terminator(Terminator::Jump {
            target: condition_block,
        });

        self.use_basic_block(condition_block);

        let condition_value = self.build_expr(ctx, *condition);
        let condition_value_type = ctx.program_builder.get_value_type(&condition_value);
        let expected_condition_type = Type {
            kind: TypeKind::Bool,
            span: condition_value_type.span,
        };

        if !self.check_is_assignable(&condition_value_type, &expected_condition_type) {
            ctx.module_builder.errors.push(SemanticError {
                span: condition_value_type.span,
                kind: SemanticErrorKind::TypeMismatch {
                    expected: expected_condition_type,
                    received: condition_value_type,
                },
            });
            // Even with an error, must continue from the merge block to keep the CFG valid
            self.use_basic_block(merge_block);
            return;
        }

        self.set_basic_block_terminator(Terminator::CondJump {
            condition: condition_value,
            true_target: body_block,
            false_target: merge_block,
        });

        self.use_basic_block(body_block);

        ctx.module_builder.enter_scope(ScopeKind::While {
            break_target: merge_block,
            continue_target: condition_block,
        });

        let _ = self.build_codeblock_expr(ctx, body);

        ctx.module_builder.exit_scope();

        self.set_basic_block_terminator(Terminator::Jump {
            target: condition_block,
        });

        self.use_basic_block(merge_block);
    }
}
