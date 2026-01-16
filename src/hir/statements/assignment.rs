use crate::{
    ast::expr::{Expr, ExprKind},
    hir::{
        cfg::{Value, ValueId},
        errors::{SemanticError, SemanticErrorKind},
        types::{checked_declaration::CheckedDeclaration, checked_type::Type},
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_lvalue_expr(
        &mut self,
        ctx: &mut HIRContext,
        expr: Expr,
    ) -> Result<(ValueId, ValueId), SemanticError> {
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

                Ok((ptr_in_block, decl.ptr))
            }
            ExprKind::Access { left, field } => {
                let (base_ptr_id, _) = self.build_lvalue_expr(ctx, *left)?;

                let field_ptr = self.emit_get_field_ptr(ctx, base_ptr_id, field)?;

                Ok((field_ptr, field_ptr))
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
        let value_span = value.span;
        let source_val = self.build_expr(ctx, value);
        let source_type = ctx.program_builder.get_value_type(&source_val);

        let (destination_ptr, root_id) = match self.build_lvalue_expr(ctx, target.clone())
        {
            Ok(ids) => ids,
            Err(e) => {
                ctx.module_builder.errors.push(e);
                return;
            }
        };

        self.emit_store(ctx, destination_ptr, source_val, value_span);

        let dest_ptr_ty = ctx.program_builder.get_value_id_type(&destination_ptr);

        if let Type::Pointer { constraint, .. } = dest_ptr_ty {
            let narrowed_ptr_ty = Type::Pointer {
                constraint,
                narrowed_to: Box::new(source_type),
            };

            let narrowed_ptr = self.emit_type_cast(
                ctx,
                Value::Use(destination_ptr),
                value_span,
                narrowed_ptr_ty,
            );

            self.map_value(self.current_block_id, root_id, narrowed_ptr);
        }
    }
}
