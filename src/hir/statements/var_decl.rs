use crate::{
    ast::{decl::VarDecl, Span},
    hir::{
        cfg::Value,
        errors::{SemanticError, SemanticErrorKind},
        types::checked_declaration::{CheckedDeclaration, CheckedVarDecl, VarStorage},
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

        // 3. Update SSA State & Declaration Data
        if let Some(val) = value_id {
            // Ensure we have a ValueId (handle literals)
            let val_id = match val {
                Value::Use(id) => todo!(),
                _ => {
                    // If build_expr returned a literal (e.g. NumberLiteral),
                    // we allocate a ValueId for it now to track it in SSA.
                    let ty = ctx.program_builder.get_value_type(&val);
                    // Note: alloc_value registers the definition in the current block
                    self.alloc_value(ctx, ty)
                }
            };

            // A. Register the SSA value for this declaration in the current block
            self.write_variable(decl_id, val_id);

            // B. Construct the Checked Declaration
            let checked_var_decl = CheckedVarDecl {
                id: decl_id,
                storage: VarStorage::Local, // Pure SSA
                identifier: var_decl.identifier,
                documentation: var_decl.documentation,
                constraint,
            };

            // C. Update the Data in ProgramBuilder
            // We overwrite the 'UninitializedVar' entry with the full 'Var' entry.
            // The Scope (Name -> ID) remains unchanged, which is exactly what we want.
            ctx.program_builder
                .declarations
                .insert(decl_id, CheckedDeclaration::Var(checked_var_decl));
        } else {
            // It remains UninitializedVar in ProgramBuilder.
            // We might want to update the constraint info if UninitializedVar supports it,
            // but for now, doing nothing keeps it as Uninitialized.
        }
    }
}
