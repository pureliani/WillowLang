use crate::{
    ast::{expr::Expr, Span},
    hir::{
        cfg::{IntrinsicFunction, Value},
        errors::{SemanticError, SemanticErrorKind},
        types::checked_type::{Type, TypeKind},
        FunctionBuilder, HIRContext,
    },
    tokenize::NumberKind,
};

impl FunctionBuilder {
    pub fn build_list_literal_expr(&mut self, ctx: &mut HIRContext, items: Vec<Expr>, expr_span: Span) -> Value {
        let item_values: Vec<Value> = items.into_iter().map(|item| self.build_expr(ctx, item)).collect();

        if item_values.is_empty() {
            let element_type = Type {
                kind: TypeKind::Void,
                span: expr_span,
            };
            let list_type = Type {
                kind: TypeKind::List(Box::new(element_type.clone())),
                span: expr_span,
            };

            let list_ptr_value_id = match self.emit_heap_alloc(ctx, element_type, Value::NumberLiteral(NumberKind::USize(0))) {
                Ok(destination) => destination,
                Err(error) => return Value::Use(self.report_error_and_get_poison(ctx, error)),
            };

            ctx.program_builder.value_types.insert(list_ptr_value_id, list_type);

            return Value::Use(list_ptr_value_id);
        }

        // TODO: refactor this first_element_type once we have unions figured out
        let first_element_type = ctx.program_builder.get_value_type(&item_values[0]);

        for item_value in item_values.iter().skip(1) {
            let item_type = ctx.program_builder.get_value_type(item_value);
            if !self.check_is_assignable(&item_type, &first_element_type) {
                return Value::Use(self.report_error_and_get_poison(
                    ctx,
                    SemanticError {
                        span: item_type.span,
                        kind: SemanticErrorKind::TypeMismatch {
                            expected: first_element_type.clone(),
                            received: item_type,
                        },
                    },
                ));
            }
        }

        let element_type = first_element_type;
        let list_type = Type {
            kind: TypeKind::List(Box::new(element_type.clone())),
            span: expr_span,
        };
        let initial_capacity = Value::NumberLiteral(NumberKind::USize(item_values.len()));
        let list_ptr_value_id = match self.emit_heap_alloc(ctx, element_type, initial_capacity) {
            Ok(destination) => destination,
            Err(error) => return Value::Use(self.report_error_and_get_poison(ctx, error)),
        };
        ctx.program_builder.value_types.insert(list_ptr_value_id, list_type);

        for (index, item) in item_values.into_iter().enumerate() {
            let result = self.emit_intrinsic_function_call(
                ctx,
                IntrinsicFunction::ListSet {
                    list_base_ptr: list_ptr_value_id,
                    index: Value::NumberLiteral(NumberKind::USize(index)),
                    item,
                },
            );
            if let Err(error) = result {
                return Value::Use(self.report_error_and_get_poison(ctx, error));
            }
        }

        Value::Use(list_ptr_value_id)
    }
}
