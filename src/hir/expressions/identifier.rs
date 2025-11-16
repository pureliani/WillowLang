use crate::{
    ast::IdentifierNode,
    hir::{
        cfg::{CheckedDeclaration, Value},
        errors::{SemanticError, SemanticErrorKind},
        types::{
            checked_declaration::{CheckedFnType, VarStorage},
            checked_type::{Type, TypeKind},
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
        match ctx.module_builder.scope_lookup(identifier.name) {
            Some(symbol_entry) => match symbol_entry.clone() {
                CheckedDeclaration::Var(checked_var_decl) => {
                    match checked_var_decl.storage {
                        VarStorage::Stack(stack_ptr) => {
                            Value::Use(self.emit_load(ctx, stack_ptr))
                        }
                        VarStorage::Captured => {
                            let env_param_decl = ctx.module_builder.scope_lookup(ctx.program_builder.env_ptr_field_name)
                                 .expect("INTERNAL COMPILER ERROR: In a closure context, but the '__env_ptr' parameter was not found in scope.");

                            let env_param_stack_ptr = if let CheckedDeclaration::Var(
                                var_decl,
                            ) = env_param_decl
                            {
                                if let VarStorage::Stack(ptr) = var_decl.storage {
                                    ptr
                                } else {
                                    panic!("INTERNAL COMPILER ERROR: Environment pointer parameter is not on the stack.");
                                }
                            } else {
                                panic!("INTERNAL COMPILER ERROR: Environment parameter declaration is not a variable.");
                            };

                            let env_ptr_id = self.emit_load(ctx, env_param_stack_ptr);

                            let field_ptr_id = match self
                                .emit_get_field_ptr(ctx, env_ptr_id, identifier)
                            {
                                Ok(ptr) => ptr,
                                Err(e) => {
                                    // This should ideally never happen if capture analysis was correct
                                    return Value::Use(
                                        self.report_error_and_get_poison(ctx, e),
                                    );
                                }
                            };

                            Value::Use(self.emit_load(ctx, field_ptr_id))
                        }
                    }
                }
                CheckedDeclaration::UninitializedVar { identifier } => {
                    return Value::Use(self.report_error_and_get_poison(
                        ctx,
                        SemanticError {
                            kind: SemanticErrorKind::UseOfUninitializedVariable(
                                identifier,
                            ),
                            span: identifier.span,
                        },
                    ));
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
                CheckedDeclaration::Function(function_id) => {
                    let fn_decl_arc = ctx.module_builder.functions.get(&function_id)
                        .expect("INTERNAL COMPILER ERROR: Function ID from scope not found in module's function list.");

                    let fn_decl = fn_decl_arc.read().unwrap();

                    let ty = Type {
                        kind: TypeKind::FnType(CheckedFnType {
                            params: fn_decl.params.clone(),
                            return_type: Box::new(fn_decl.return_type.clone()),
                        }),
                        span: fn_decl.identifier.span,
                    };

                    Value::FunctionAddr {
                        function_id: fn_decl.function_id,
                        ty,
                    }
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
