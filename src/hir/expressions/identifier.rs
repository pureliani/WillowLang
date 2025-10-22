use crate::{
    ast::IdentifierNode,
    hir::{
        cfg::{CheckedDeclaration, Value},
        errors::{SemanticError, SemanticErrorKind},
        types::checked_declaration::VarStorage,
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
            Some(symbol_entry) => match symbol_entry.clone() {
                CheckedDeclaration::Var(checked_var_decl) => {
                    match checked_var_decl.storage {
                        VarStorage::Stack(stack_ptr) => {
                            Value::Use(self.emit_load(ctx, stack_ptr))
                        }
                        VarStorage::Captured => {
                            todo!("Implement access for a captured variable");
                        }
                    }
                }
                CheckedDeclaration::TypeAlias(decl) => {
                    let span = decl.read().unwrap().identifier.span;

                    Value::Use(self.report_error_and_get_poison(
                        ctx,
                        SemanticError {
                            kind: SemanticErrorKind::CannotUseTypeDeclarationAsValue,
                            span,
                        },
                    ))
                }
                CheckedDeclaration::Function(_) => {
                    todo!("Handle function as a value");
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
