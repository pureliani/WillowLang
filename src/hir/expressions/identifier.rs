use crate::{
    ast::IdentifierNode,
    hir::{
        cfg::Value,
        errors::{SemanticError, SemanticErrorKind},
        types::checked_declaration::{CheckedEnumDecl, CheckedTypeAliasDecl},
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
                    let var_ptr_id = checked_var_decl.stack_ptr;
                    let loaded_value_id = self.emit_load(ctx, var_ptr_id);
                    Value::Use(loaded_value_id)
                }
                SymbolEntry::TypeAliasDecl(CheckedTypeAliasDecl {
                    identifier, ..
                })
                | SymbolEntry::EnumDecl(CheckedEnumDecl { identifier, .. }) => {
                    Value::Use(self.report_error_and_get_poison(
                        ctx,
                        SemanticError {
                            kind: SemanticErrorKind::CannotUseTypeDeclarationAsValue,
                            span: identifier.span,
                        },
                    ))
                }
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
