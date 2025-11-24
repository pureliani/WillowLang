use std::collections::{HashMap, HashSet};

use crate::{
    ast::{expr::Expr, IdentifierNode, Span},
    hir::{
        cfg::Value,
        errors::{SemanticError, SemanticErrorKind},
        types::{
            checked_declaration::CheckedParam,
            checked_type::{StructKind, Type},
        },
        utils::layout::pack_struct,
        FunctionBuilder, HIRContext,
    },
    tokenize::NumberKind,
};

impl FunctionBuilder {
    pub fn build_struct_init_expr(
        &mut self,
        ctx: &mut HIRContext,
        fields: Vec<(IdentifierNode, Expr)>,
        span: Span,
    ) -> Value {
        let mut resolved_fields: Vec<CheckedParam> = Vec::with_capacity(fields.len());
        let mut field_values: HashMap<IdentifierNode, Value> =
            HashMap::with_capacity(fields.len());
        let mut initialized_fields: HashSet<IdentifierNode> = HashSet::new();

        for (field_name, field_expr) in fields {
            if !initialized_fields.insert(field_name) {
                return Value::Use(self.report_error_and_get_poison(
                    ctx,
                    SemanticError {
                        kind: SemanticErrorKind::DuplicateStructFieldInitializer(
                            field_name,
                        ),
                        span: field_name.span,
                    },
                ));
            }

            let value = self.build_expr(ctx, field_expr);
            let value_type = ctx.program_builder.get_value_type(&value);

            resolved_fields.push(CheckedParam {
                identifier: field_name,
                ty: value_type,
            });
            field_values.insert(field_name, value);
        }

        pack_struct(&ctx.program_builder, &mut resolved_fields);

        let struct_type = Type::Struct(StructKind::UserDefined(resolved_fields));

        let struct_ptr = self
            .emit_heap_alloc(
                ctx,
                struct_type.clone(),
                Value::NumberLiteral(NumberKind::USize(1)),
            )
            .expect("INTERNAL COMPILER ERROR: failed to allocate struct on heap");

        if let Type::Struct(StructKind::UserDefined(sorted_fields)) = &struct_type {
            for field in sorted_fields {
                let field_ptr =
                    match self.emit_get_field_ptr(ctx, struct_ptr, field.identifier) {
                        Ok(ptr) => ptr,
                        Err(error) => {
                            return Value::Use(
                                self.report_error_and_get_poison(ctx, error),
                            )
                        }
                    };

                let field_value = field_values.get(&field.identifier).unwrap();

                self.emit_store(ctx, field_ptr, field_value.clone());
            }
        }

        Value::Use(struct_ptr)
    }
}
