use crate::{
    ast::{decl::TypeAliasDecl, Span},
    hir::{
        errors::{SemanticError, SemanticErrorKind},
        types::checked_declaration::CheckedTypeAliasDecl,
        utils::scope::{ScopeKind, SymbolEntry},
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

        ctx.module_builder.enter_scope(ScopeKind::TypeAlias);
        let alias_value =
            Box::new(self.check_type_annotation(ctx, &type_alias_decl.value));
        ctx.module_builder.exit_scope();

        let checked_type_alias_decl = CheckedTypeAliasDecl {
            documentation: type_alias_decl.documentation,
            identifier: type_alias_decl.identifier,
            span,
            value: alias_value,
        };

        ctx.module_builder.scope_insert(
            type_alias_decl.identifier,
            SymbolEntry::TypeAliasDecl(checked_type_alias_decl),
            span,
        );
    }
}
