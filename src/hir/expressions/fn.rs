use std::sync::{Arc, RwLock};

use crate::{
    ast::{decl::FnDecl, expr::BlockContents},
    hir::{
        cfg::{CheckedDeclaration, Terminator, Value},
        errors::{SemanticError, SemanticErrorKind},
        types::{
            checked_declaration::{
                CheckedFnDecl, CheckedFnType, CheckedParam, CheckedVarDecl, VarStorage,
            },
            checked_type::{Type, TypeKind},
        },
        utils::scope::ScopeKind,
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
                storage: VarStorage::Stack(variable_stack_ptr),
                identifier: param.identifier,
                documentation: None,
                constraint: param.ty.clone(),
            };

            ctx.module_builder.scope_insert(
                param.identifier,
                CheckedDeclaration::Var(checked_var_decl),
                param.identifier.span,
            );
        }

        let final_value = self.build_codeblock_expr(ctx, body);
        let final_value_type = ctx.program_builder.get_value_type(&final_value);

        if !self.check_is_assignable(&final_value_type, &self.return_type) {
            ctx.module_builder.errors.push(SemanticError {
                span: final_value_type.span,
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

        let new_function_id = ctx.program_builder.new_function_id();
        let checked_fn_decl = Arc::new(RwLock::new(CheckedFnDecl {
            id: ctx.program_builder.new_declaration_id(),
            identifier,
            params: checked_params.clone(),
            return_type: checked_return_type.clone(),
            body: None,
        }));

        ctx.module_builder.scope_insert(
            identifier,
            CheckedDeclaration::Function(checked_fn_decl.clone()),
            identifier.span,
        );

        let mut new_fn_builder = FunctionBuilder::new(checked_return_type.clone());

        ctx.module_builder
            .enter_scope(ScopeKind::Function(Box::new(new_fn_builder)));

        let active_builder = ctx.module_builder.get_active_function_builder();
        active_builder.build_fn_body(ctx, &checked_params, body);

        let scope = ctx.module_builder.exit_scope();
        let final_fn_builder = match scope.kind {
            ScopeKind::Function(builder) => builder,
            _ => panic!("INTERNAL COMPILER ERROR: Expected to pop a function scope."),
        };

        let mut fn_decl_writer = checked_fn_decl.write().unwrap();
        fn_decl_writer.body = Some(final_fn_builder.cfg);

        let fn_type = Type {
            kind: TypeKind::FnType(CheckedFnType {
                params: checked_params,
                return_type: Box::new(checked_return_type),
            }),
            span: identifier.span,
        };

        Value::FunctionAddr {
            function_id: new_function_id,
            ty: fn_type,
        }
    }
}
