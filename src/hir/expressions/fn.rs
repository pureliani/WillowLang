use std::sync::{Arc, RwLock};

use crate::{
    ast::{
        decl::Param, expr::BlockContents, type_annotation::TypeAnnotation, IdentifierNode,
    },
    hir::{
        cfg::{CheckedDeclaration, Value},
        types::checked_declaration::{CheckedFnDecl, CheckedParam},
        utils::scope::ScopeKind,
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_fn_expr(
        &mut self,
        ctx: &mut HIRContext,
        params: Vec<Param>,
        body: BlockContents,
        return_type: TypeAnnotation,
        name: IdentifierNode,
    ) -> Value {
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
            identifier: name,
            params: checked_params.clone(),
            return_type: checked_return_type.clone(),
            body: None,
        }));

        ctx.module_builder.scope_insert(
            name,
            CheckedDeclaration::Function(checked_fn_decl.clone()),
            name.span,
        );

        let mut new_fn_builder = FunctionBuilder::new(checked_return_type.clone());
        ctx.module_builder
            .enter_scope(ScopeKind::Function(Box::new(new_fn_builder)));

        todo!()
    }
}
