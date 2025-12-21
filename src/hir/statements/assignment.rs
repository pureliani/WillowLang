use crate::{
    ast::expr::{Expr, ExprKind},
    hir::{
        cfg::ValueId,
        errors::{SemanticError, SemanticErrorKind},
        types::checked_declaration::CheckedDeclaration,
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_lvalue_expr(
        &mut self,
        ctx: &mut HIRContext,
        expr: Expr,
    ) -> Result<ValueId, SemanticError> {
        match expr.kind {
            ExprKind::Identifier(identifier) => {
                let id = ctx.module_builder.scope_lookup(identifier.name);
                let declaration = id.map(|id| ctx.program_builder.get_declaration(id));

                let decl = match declaration {
                    Some(CheckedDeclaration::Var(var_decl)) => Ok(var_decl.clone()),
                    Some(_) => Err(SemanticError {
                        kind: SemanticErrorKind::InvalidLValue,
                        span: expr.span,
                    }),
                    None => Err(SemanticError {
                        kind: SemanticErrorKind::UndeclaredIdentifier(identifier),
                        span: expr.span,
                    }),
                }?;

                let ptr_in_block =
                    self.use_value_in_block(ctx, self.current_block_id, decl.ptr);
                Ok(ptr_in_block)
            }
            ExprKind::Access { left, field } => {
                let base_ptr_id = self.build_lvalue_expr(ctx, *left)?;
                self.emit_get_field_ptr(ctx, base_ptr_id, field)
            }
            _ => Err(SemanticError {
                kind: SemanticErrorKind::InvalidLValue,
                span: expr.span,
            }),
        }
    }

    pub fn build_assignment_stmt(
        &mut self,
        ctx: &mut HIRContext,
        target: Expr,
        value: Expr,
    ) {
        let source_val = self.build_expr(ctx, value);

        let destination_ptr = match self.build_lvalue_expr(ctx, target) {
            Ok(value_id) => value_id,
            Err(e) => {
                ctx.module_builder.errors.push(e);
                return;
            }
        };

        self.emit_store(ctx, destination_ptr, source_val);
    }
}
