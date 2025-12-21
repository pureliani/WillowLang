use crate::{
    ast::{decl::VarDecl, Span},
    hir::{
        cfg::Value,
        errors::{SemanticError, SemanticErrorKind},
        types::checked_declaration::{CheckedDeclaration, CheckedVarDecl},
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_var_decl(
        &mut self,
        ctx: &mut HIRContext,
        var_decl: VarDecl,
        span: Span,
    ) {
        if ctx.module_builder.is_file_scope() {
            ctx.module_builder.errors.push(SemanticError {
                kind: SemanticErrorKind::CannotDeclareGlobalVariable,
                span,
            });
            return;
        }

        let decl_id = ctx
            .module_builder
            .scope_lookup(var_decl.identifier.name)
            .expect("INTERNAL COMPILER ERROR: Variable declaration was not hoisted");

        let (value_id, constraint) = match var_decl.constraint {
            Some(constraint_annotation) => match var_decl.value {
                Some(value_expr) => {
                    let initial_value = self.build_expr(ctx, value_expr);
                    let initial_value_type =
                        ctx.program_builder.get_value_type(&initial_value);

                    let expected_constraint =
                        self.check_type_annotation(ctx, &constraint_annotation);

                    if !self
                        .check_is_assignable(&initial_value_type, &expected_constraint)
                    {
                        ctx.module_builder.errors.push(SemanticError {
                            span: Default::default(), // TODO: Add appropriate span
                            kind: SemanticErrorKind::TypeMismatch {
                                expected: expected_constraint,
                                received: initial_value_type,
                            },
                        });
                        return;
                    }

                    (Some(initial_value), expected_constraint)
                }
                None => {
                    let expected_constraint =
                        self.check_type_annotation(ctx, &constraint_annotation);
                    (None, expected_constraint)
                }
            },
            None => match var_decl.value {
                Some(value_expr) => {
                    let initial_value = self.build_expr(ctx, value_expr);
                    let initial_value_type =
                        ctx.program_builder.get_value_type(&initial_value);

                    (Some(initial_value), initial_value_type)
                }
                None => {
                    ctx.module_builder.errors.push(SemanticError {
                        kind: SemanticErrorKind::VarDeclWithoutConstraintOrInitializer,
                        span,
                    });
                    return;
                }
            },
        };

        if let Some(val) = value_id {
            let ptr = self.emit_stack_alloc(ctx, constraint.clone(), 1);

            let val_id = match val {
                Value::Use(id) => self.use_value_in_block(ctx, self.current_block_id, id),
                _ => {
                    let ty = ctx.program_builder.get_value_type(&val);
                    self.emit_type_cast(ctx, val, ty)
                }
            };

            let checked_var_decl = CheckedVarDecl {
                id: decl_id,
                ptr,
                identifier: var_decl.identifier,
                documentation: var_decl.documentation,
                constraint,
            };

            self.emit_store(ctx, ptr, Value::Use(val_id));

            ctx.program_builder
                .declarations
                .insert(decl_id, CheckedDeclaration::Var(checked_var_decl));
        }
    }
}
