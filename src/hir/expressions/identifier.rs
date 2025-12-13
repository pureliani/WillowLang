use crate::{
    ast::IdentifierNode,
    hir::{
        cfg::Value,
        errors::{SemanticError, SemanticErrorKind},
        types::{
            checked_declaration::{CheckedDeclaration, FnType, VarStorage},
            checked_type::Type,
        },
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
                    match checked_var_decl.storage {
                        VarStorage::Local => {
                            let v = self.read_variable(ctx, checked_var_decl.id);
                            Value::Use(v)
                        }
                        VarStorage::Heap(ptr) => {
                            todo!()
                        }
                    }
                }
                CheckedDeclaration::UninitializedVar { id, identifier } => {
                    return Value::Use(self.report_error_and_get_poison(
                        ctx,
                        SemanticError {
                            kind: SemanticErrorKind::UseOfUninitializedVariable(
                                identifier.clone(),
                            ),
                            span: identifier.span,
                        },
                    ));
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
                    todo!()
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
