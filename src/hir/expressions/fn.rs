use crate::{
    ast::{decl::FnDecl, expr::BlockContents, Span},
    hir::{
        cfg::{Terminator, Value},
        errors::{SemanticError, SemanticErrorKind},
        types::checked_declaration::{CheckedDeclaration, CheckedParam, CheckedVarDecl},
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_fn_body(
        &mut self,
        ctx: &mut HIRContext,
        params: &[CheckedParam],
        body: BlockContents,
    ) {
        for param in params {
            let variable_stack_ptr = self.emit_stack_alloc(ctx, param.ty.clone(), 1);

            // The calling convention will be responsible for storing the actual argument value
            // into this variable's stack slot before the function body begins execution.

            let checked_var_decl = CheckedVarDecl {
                id: ctx.program_builder.new_declaration_id(),
                ptr: variable_stack_ptr,
                identifier: param.identifier,
                documentation: None,
                constraint: param.ty.clone(),
            };

            ctx.module_builder.scope_insert(
                ctx.program_builder,
                param.identifier,
                CheckedDeclaration::Var(checked_var_decl),
            );
        }

        let final_value = self.build_codeblock_expr(ctx, body);
        let final_value_type = ctx.program_builder.get_value_type(&final_value);

        if !self.check_is_assignable(&final_value_type, &self.return_type) {
            ctx.module_builder.errors.push(SemanticError {
                span: Span::default(), // TODO: fix later
                kind: SemanticErrorKind::ReturnTypeMismatch {
                    expected: self.return_type.clone(),
                    received: final_value_type,
                },
            });

            self.set_basic_block_terminator(Terminator::Unreachable);
            return;
        }

        self.set_basic_block_terminator(Terminator::Return {
            value: Some(final_value),
        });
    }

    pub fn build_fn_expr(&mut self, ctx: &mut HIRContext, fn_decl: FnDecl) -> Value {
        let FnDecl {
            identifier,
            params,
            return_type,
            body,
        } = fn_decl;

        let checked_params: Vec<CheckedParam> = params
            .iter()
            .map(|p| CheckedParam {
                identifier: p.identifier,
                ty: self.check_type_annotation(ctx, &p.constraint),
            })
            .collect();
        let checked_return_type = self.check_type_annotation(ctx, &return_type);

        todo!()
    }
}
