use std::collections::HashSet;

use crate::{
    ast::{
        expr::{Expr, ExprKind},
        IdentifierNode,
    },
    compile::string_interner::InternerId,
    hir::{
        cfg::Value,
        errors::{SemanticError, SemanticErrorKind},
        types::{
            checked_declaration::CheckedStructDecl,
            checked_type::{Type, TypeKind},
        },
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_capture_struct_for_fn(&mut self) -> CheckedStructDecl {
        todo!()
    }

    fn evaluate_expression_with_type_tail(&mut self, ctx: &mut HIRContext, expr: Expr) -> Result<Type, SemanticError> {
        let expr_span = expr.span;
        match expr.kind {
            ExprKind::Identifier(id) => Ok(Type {
                span: expr.span,
                kind: self.check_type_identifier_annotation(ctx, id, expr.span)?,
            }),
            ExprKind::StaticAccess { left, field } => {
                let left_span = left.span;
                let left_type = self.evaluate_expression_with_type_tail(ctx, *left)?;

                match left_type.kind {
                    TypeKind::FnType(fn_decl) => {
                        if field.name == ctx.program_builder.string_interner.intern("env") {
                            let capture_struct_decl = todo!();
                            Ok(Type {
                                kind: TypeKind::Struct(capture_struct_decl),
                                span: expr_span,
                            })
                        } else {
                            Err(SemanticError {
                                kind: SemanticErrorKind::AccessToUndefinedStaticField(field),
                                span: field.span,
                            })
                        }
                    }
                    _ => Err(SemanticError {
                        span: left_span,
                        kind: SemanticErrorKind::CannotStaticAccess(left_type),
                    }),
                }
            }
            _ => Err(SemanticError {
                kind: SemanticErrorKind::ExpectedAType,
                span: expr_span,
            }),
        }
    }

    pub fn build_struct_init_expr(
        &mut self,
        ctx: &mut HIRContext,
        left: Box<Expr>,
        field_initializers: Vec<(IdentifierNode, Expr)>,
    ) -> Value {
        let type_expr_span = left.span;
        let left_type = match self.evaluate_expression_with_type_tail(ctx, *left) {
            Ok(t) => t,
            Err(e) => return Value::Use(self.report_error_and_get_poison(ctx, e)),
        };

        if let TypeKind::Struct(struct_decl) = &left_type.kind {
            let mut initialized_fields: HashSet<IdentifierNode> = HashSet::new();

            let struct_ptr = self.emit_stack_alloc(ctx, left_type.clone(), 1);

            for (field_name, field_expr) in field_initializers {
                if !initialized_fields.insert(field_name) {
                    return Value::Use(self.report_error_and_get_poison(
                        ctx,
                        SemanticError {
                            kind: SemanticErrorKind::DuplicateStructFieldInitializer(field_name),
                            span: field_name.span,
                        },
                    ));
                }

                let field_expr_span = field_expr.span;

                let field_ptr = match self.emit_get_field_ptr(ctx, struct_ptr, field_name) {
                    Ok(ptr) => ptr,
                    Err(error) => return Value::Use(self.report_error_and_get_poison(ctx, error)),
                };
                let field_ptr_type = ctx.program_builder.get_value_id_type(&field_ptr);

                let field_value = self.build_expr(ctx, field_expr);
                let field_value_type = ctx.program_builder.get_value_type(&field_value);

                if let TypeKind::Pointer(expected_field_type) = field_ptr_type.kind {
                    if !self.check_is_assignable(&field_value_type, &expected_field_type) {
                        return Value::Use(self.report_error_and_get_poison(
                            ctx,
                            SemanticError {
                                span: field_expr_span,
                                kind: SemanticErrorKind::TypeMismatch {
                                    expected: *expected_field_type,
                                    received: field_value_type,
                                },
                            },
                        ));
                    }
                } else {
                    panic!("INTERNAL COMPILER ERROR: struct field pointer must be a pointer");
                }

                self.emit_store(ctx, field_ptr, field_value);
            }

            let mut missing_initializers: HashSet<InternerId> = HashSet::new();
            for required_field in struct_decl.fields() {
                if !initialized_fields.contains(&required_field.identifier) {
                    missing_initializers.insert(required_field.identifier.name);
                }
            }
            if !missing_initializers.is_empty() {
                return Value::Use(self.report_error_and_get_poison(
                    ctx,
                    SemanticError {
                        kind: SemanticErrorKind::MissingStructFieldInitializers(missing_initializers),
                        span: type_expr_span,
                    },
                ));
            }

            let final_struct_value_id = self.emit_load(ctx, struct_ptr);
            return Value::Use(final_struct_value_id);
        } else {
            return Value::Use(self.report_error_and_get_poison(
                ctx,
                SemanticError {
                    span: type_expr_span,
                    kind: SemanticErrorKind::CannotApplyStructInitializer,
                },
            ));
        }
    }
}
