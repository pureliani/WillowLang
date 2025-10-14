use crate::{
    ast::IdentifierNode,
    hir::{
        cfg::Value,
        errors::{SemanticError, SemanticErrorKind},
        types::checked_declaration::CheckedTypeAliasDecl,
        utils::scope::SymbolEntry,
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_identifier_expr(
        &mut self,
        ctx: &mut HIRContext,
        identifier: IdentifierNode,
    ) -> Value {
        match ctx.module_builder.scope_lookup(identifier.name) {
            Some(symbol_entry) => match symbol_entry {
                SymbolEntry::VarDecl(checked_var_decl) => {
                    let data = self.emit_load(ctx, checked_var_decl.stack_ptr);
                    Value::Use(data)
                }
                SymbolEntry::TypeAliasDecl(CheckedTypeAliasDecl {
                    identifier, ..
                }) => Value::Use(self.report_error_and_get_poison(
                    ctx,
                    SemanticError {
                        kind: SemanticErrorKind::CannotUseTypeDeclarationAsValue,
                        span: identifier.span,
                    },
                )),
            },
            None => Value::Use(self.report_error_and_get_poison(
                ctx,
                SemanticError {
                    kind: SemanticErrorKind::UndeclaredIdentifier(identifier),
                    span: identifier.span,
                },
            )),
        }
    }
}
