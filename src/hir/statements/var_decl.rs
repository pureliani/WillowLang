use std::sync::{Arc, RwLock};

use crate::{
    ast::{decl::VarDecl, Span},
    hir::{
        cfg::CheckedDeclaration,
        errors::{SemanticError, SemanticErrorKind},
        types::checked_declaration::{CheckedVarDecl, VarStorage},
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

        let (value, constraint) = match var_decl.constraint {
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
                            span: initial_value_type.span,
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

        let variable_stack_ptr = self.emit_stack_alloc(ctx, constraint.clone(), 1);
        if let Some(initial_value) = value {
            self.emit_store(ctx, variable_stack_ptr, initial_value);
        };

        let checked_var_decl = CheckedVarDecl {
            id: ctx.program_builder.new_declaration_id(),
            storage: VarStorage::Stack(variable_stack_ptr),
            identifier: var_decl.identifier,
            documentation: var_decl.documentation,
            constraint,
        };

        ctx.module_builder.scope_insert(
            var_decl.identifier,
            CheckedDeclaration::Var(checked_var_decl),
            span,
        );
    }
}
