use crate::{
    ast::expr::{Expr, ExprKind},
    hir::{
        cfg::ValueId,
        errors::{SemanticError, SemanticErrorKind},
        utils::scope::SymbolEntry,
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
                if let Some(SymbolEntry::VarDecl(decl)) =
                    ctx.module_builder.scope_lookup(identifier.name)
                {
                    return Ok(decl.stack_ptr); // ValueId which holds Pointer<T>
                } else {
                    return Err(SemanticError {
                        kind: SemanticErrorKind::UndeclaredIdentifier(identifier),
                        span: expr.span,
                    });
                }
            }
            ExprKind::Access { left, field } => {
                let base_ptr_id = self.build_lvalue_expr(ctx, *left)?;
                Ok(self.emit_get_field_ptr(ctx, base_ptr_id, field)?)
            }
            _ => {
                return Err(SemanticError {
                    kind: SemanticErrorKind::InvalidLValue,
                    span: expr.span,
                });
            }
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
