use crate::{
    ast::{decl::TypeAliasDecl, Span},
    hir::{
        errors::{SemanticError, SemanticErrorKind},
        types::checked_declaration::{CheckedDeclaration, CheckedTypeAliasDecl},
        utils::check_type::check_type_annotation,
        HIRContext,
    },
};

pub fn build_type_alias_decl(
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

    let alias_value = Box::new(check_type_annotation(ctx, &type_alias_decl.value));

    let checked_type_alias_decl = CheckedTypeAliasDecl {
        id: type_alias_decl.id,
        documentation: type_alias_decl.documentation,
        identifier: type_alias_decl.identifier,
        span,
        value: alias_value,
        is_exported: type_alias_decl.is_exported,
    };

    ctx.module_builder.scope_insert(
        ctx.program_builder,
        type_alias_decl.identifier,
        CheckedDeclaration::TypeAlias(checked_type_alias_decl),
    );
}
