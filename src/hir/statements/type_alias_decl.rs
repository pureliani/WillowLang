use std::sync::{Arc, RwLock};

use crate::{
    ast::{decl::TypeAliasDecl, Span},
    hir::{
        cfg::CheckedDeclaration,
        errors::{SemanticError, SemanticErrorKind},
        types::checked_declaration::CheckedTypeAliasDecl,
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_type_alias_decl(
        &mut self,
        ctx: &mut HIRContext,
        type_alias_decl: TypeAliasDecl,
        span: Span,
    ) {
        if !ctx.module_builder.is_file_scope() {
            ctx.module_builder.errors.push(SemanticError {
                kind: SemanticErrorKind::TypeAliasMustBeDeclaredAtTopLevel,
                span,
            });
            return;
        }

        let alias_value =
            Box::new(self.check_type_annotation(ctx, &type_alias_decl.value));

        let checked_type_alias_decl = Arc::new(RwLock::new(CheckedTypeAliasDecl {
            id: ctx.program_builder.new_declaration_id(),
            documentation: type_alias_decl.documentation,
            identifier: type_alias_decl.identifier,
            span,
            value: alias_value,
        }));

        ctx.module_builder.scope_insert(
            type_alias_decl.identifier,
            CheckedDeclaration::TypeAlias(checked_type_alias_decl),
            span,
        );
    }
}
