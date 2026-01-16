use crate::{
    ast::IdentifierNode,
    hir::{
        cfg::Value,
        errors::{SemanticError, SemanticErrorKind},
        types::checked_declaration::CheckedDeclaration,
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_identifier_expr(
        &mut self,
        ctx: &mut HIRContext,
        identifier: IdentifierNode,
    ) -> Value {
        let maybe_decl = ctx
            .module_builder
            .scope_lookup(identifier.name)
            .map(|id| ctx.program_builder.get_declaration(id));

        match maybe_decl {
            Some(decl) => match decl {
                CheckedDeclaration::Var(checked_var_decl) => {
                    let ptr_val = self.use_value_in_block(
                        ctx,
                        self.current_block_id,
                        checked_var_decl.ptr,
                    );

                    Value::Use(self.emit_load(ctx, ptr_val))
                }
                CheckedDeclaration::UninitializedVar { identifier, .. } => {
                    Value::Use(self.report_error_and_get_poison(
                        ctx,
                        SemanticError {
                            kind: SemanticErrorKind::UseOfUninitializedVariable(
                                *identifier,
                            ),
                            span: identifier.span,
                        },
                    ))
                }
                CheckedDeclaration::TypeAlias(decl) => {
                    Value::Use(self.report_error_and_get_poison(
                        ctx,
                        SemanticError {
                            kind: SemanticErrorKind::CannotUseTypeDeclarationAsValue,
                            span: decl.identifier.span,
                        },
                    ))
                }
                CheckedDeclaration::Function(checked_fn_decl) => {
                    Value::Function(checked_fn_decl.id)
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
