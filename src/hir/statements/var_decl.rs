use crate::{
    ast::{decl::VarDecl, Span},
    hir::{
        cfg::Value,
        errors::{SemanticError, SemanticErrorKind},
        types::checked_declaration::{CheckedDeclaration, CheckedVarDecl},
        utils::check_type::check_type_annotation,
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

        let (initial_value, initial_constraint) = match var_decl.constraint {
            Some(constraint_annotation) => {
                let initial_value_span = var_decl.value.span;
                let initial_value = self.build_expr(ctx, var_decl.value);
                let initial_value_type =
                    ctx.program_builder.get_value_type(&initial_value);

                let expected_constraint =
                    check_type_annotation(ctx, &constraint_annotation);

                if !self.check_is_assignable(&initial_value_type, &expected_constraint) {
                    ctx.module_builder.errors.push(SemanticError {
                        span: initial_value_span,
                        kind: SemanticErrorKind::TypeMismatch {
                            expected: expected_constraint,
                            received: initial_value_type,
                        },
                    });
                    return;
                }

                (initial_value, expected_constraint)
            }
            None => {
                let initial_value = self.build_expr(ctx, var_decl.value);
                let initial_value_type =
                    ctx.program_builder.get_value_type(&initial_value);

                (initial_value, initial_value_type)
            }
        };

        let ptr = self.emit_stack_alloc(ctx, initial_constraint.clone(), 1);

        let val_id = match initial_value {
            Value::Use(id) => self.use_value_in_block(ctx, self.current_block_id, id),
            _ => {
                let ty = ctx.program_builder.get_value_type(&initial_value);
                self.emit_type_cast(ctx, initial_value, ty)
            }
        };

        let checked_var_decl = CheckedVarDecl {
            id: ctx.program_builder.new_declaration_id(),
            ptr,
            identifier: var_decl.identifier,
            documentation: var_decl.documentation,
            constraint: initial_constraint,
        };

        self.emit_store(ctx, ptr, Value::Use(val_id));

        ctx.module_builder.scope_insert(
            ctx.program_builder,
            var_decl.identifier,
            CheckedDeclaration::Var(checked_var_decl),
        );
    }
}
