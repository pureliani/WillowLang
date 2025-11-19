use crate::{
    ast::{expr::Expr, IdentifierNode, Span},
    hir::{
        cfg::{IntrinsicFunction, Value},
        errors::{SemanticError, SemanticErrorKind},
        types::{
            checked_declaration::CheckedParam,
            checked_type::{CheckedStruct, StructKind, Type, TypeKind},
        },
        FunctionBuilder, HIRContext,
    },
    tokenize::NumberKind,
};
use std::collections::HashSet;

impl FunctionBuilder {
    /// Helper to create the specific List<T> struct type: { capacity: usize, len: usize, ptr: *T }
    fn create_list_type(
        &self,
        ctx: &mut HIRContext,
        element_type: Type,
        span: Span,
    ) -> Type {
        let usize_type = Type {
            kind: TypeKind::USize,
            span,
        };
        let ptr_type = Type {
            kind: TypeKind::Pointer(Box::new(element_type)),
            span,
        };

        let cap_id = ctx.program_builder.common_identifiers.capacity;
        let len_id = ctx.program_builder.common_identifiers.len;
        let ptr_id = ctx.program_builder.common_identifiers.ptr;

        let fields = vec![
            CheckedParam {
                identifier: IdentifierNode { name: cap_id, span },
                ty: usize_type.clone(),
            },
            CheckedParam {
                identifier: IdentifierNode { name: len_id, span },
                ty: usize_type,
            },
            CheckedParam {
                identifier: IdentifierNode { name: ptr_id, span },
                ty: ptr_type,
            },
        ];

        Type {
            kind: TypeKind::Struct(CheckedStruct {
                kind: StructKind::List,
                fields,
            }),
            span,
        }
    }

    pub fn build_list_literal_expr(
        &mut self,
        ctx: &mut HIRContext,
        items: Vec<Expr>,
        expr_span: Span,
    ) -> Value {
        let item_values: Vec<Value> = items
            .into_iter()
            .map(|item| self.build_expr(ctx, item))
            .collect();

        let element_type = if item_values.is_empty() {
            Type {
                kind: TypeKind::Void,
                span: expr_span,
            }
        } else {
            // Check if all items are Tags to infer a Union
            let all_tags = item_values.iter().all(|v| {
                let ty = ctx.program_builder.get_value_type(v);
                matches!(
                    ty.kind,
                    TypeKind::Struct(CheckedStruct {
                        kind: StructKind::Tag,
                        ..
                    })
                )
            });

            if all_tags {
                // Collect unique tag types to form a Union
                let mut union_variants = Vec::new();
                let mut seen_tags = HashSet::new();

                for val in &item_values {
                    let ty = ctx.program_builder.get_value_type(val);
                    if let TypeKind::Pointer(ptr) = &ty.kind {
                        if let TypeKind::Struct(CheckedStruct {
                            kind: StructKind::Tag,
                            fields,
                        }) = &ptr.kind
                        {
                            // We assume the first field is the discriminant/identifier for uniqueness
                            // This logic depends on how you structure your Tag fields.
                            // For now, we just add them if the Type is unique.
                            // A better way is to check the Tag's name/discriminant.
                            // Assuming `fields` contains the tag info.
                            self.emit_get_field_ptr(
                                ctx,
                                val,
                                IdentifierNode {
                                    name: ctx.program_builder.common_identifiers.id,
                                    span: Span::default(),
                                },
                            );

                            let id = fields.get(0).expect("INTERNAL COMPILER ERROR: Expected a tag struct to have an id field");

                            // Ideally, you'd check if this specific tag variant is already in `union_variants`
                            // For simplicity here, we just push. You should deduplicate in a real implementation.
                            if !seen_tags.contains(&ty) {
                                // Type needs to implement Hash/Eq properly
                                // We need to extract the "Variant" definition from the Tag instance
                                // to put into the Union definition.
                                // For now, we treat the Tag Type itself as the variant definition.
                                // In your StructKind::Union, `fields` might actually be a list of Tag Types?
                                // Or a list of Params where each Param is a Tag?
                                // Let's assume Union fields are just the Tag types.

                                // NOTE: This part depends heavily on how you decided to represent Union fields.
                                // If Union fields are just a list of possible Tag Structs:
                                union_variants.extend(fields);
                                seen_tags.insert(ty);
                            }
                        }
                    }
                }

                Type {
                    kind: TypeKind::Struct(CheckedStruct {
                        kind: StructKind::Union,
                        fields: union_variants,
                    }),
                    span: expr_span,
                }
            } else {
                // Standard Homogeneous List
                let first_type = ctx.program_builder.get_value_type(&item_values[0]);

                // Verify all other items match the first type
                for item_value in item_values.iter().skip(1) {
                    let item_type = ctx.program_builder.get_value_type(item_value);
                    if !self.check_is_assignable(&item_type, &first_type) {
                        return Value::Use(self.report_error_and_get_poison(
                            ctx,
                            SemanticError {
                                span: item_type.span,
                                kind: SemanticErrorKind::TypeMismatch {
                                    expected: first_type.clone(),
                                    received: item_type,
                                },
                            },
                        ));
                    }
                }
                first_type
            }
        };

        // 2. Create the List Type (Struct)
        let list_type = self.create_list_type(ctx, element_type.clone(), expr_span);

        // 3. Allocate the Buffer (The array of elements)
        let capacity_val = Value::NumberLiteral(NumberKind::USize(item_values.len()));

        // Note: emit_heap_alloc returns a Pointer<element_type>
        let buffer_ptr = match self.emit_heap_alloc(ctx, element_type, capacity_val) {
            Ok(dest) => dest,
            Err(e) => return Value::Use(self.report_error_and_get_poison(ctx, e)),
        };

        // 4. Allocate the List Header (The struct {cap, len, ptr})
        // We allocate 1 instance of the List Struct
        let list_header_ptr = match self.emit_heap_alloc(
            ctx,
            list_type.clone(),
            Value::NumberLiteral(NumberKind::USize(1)),
        ) {
            Ok(dest) => dest,
            Err(e) => return Value::Use(self.report_error_and_get_poison(ctx, e)),
        };

        // 5. Initialize the List Header Fields
        // We need to store: capacity, len, and the buffer_ptr into the list_header_ptr

        // Helper to get field ptr and store
        let store_field = |builder: &mut FunctionBuilder,
                           ctx: &mut HIRContext,
                           field_name: &str,
                           val: Value| {
            let name_id = ctx.program_builder.string_interner.intern(field_name);
            let id_node = IdentifierNode {
                name: name_id,
                span: expr_span,
            };

            // This might fail if create_list_type didn't use the exact same interned ID
            // Ensure create_list_type uses the same interner!
            let field_ptr = builder
                .emit_get_field_ptr(ctx, list_header_ptr, id_node)
                .unwrap();
            builder.emit_store(ctx, field_ptr, val);
        };

        store_field(
            self,
            ctx,
            "capacity",
            Value::NumberLiteral(NumberKind::USize(item_values.len())),
        );
        store_field(
            self,
            ctx,
            "len",
            Value::NumberLiteral(NumberKind::USize(item_values.len())),
        );
        store_field(self, ctx, "ptr", Value::Use(buffer_ptr));

        // 6. Populate the Buffer
        // We use the IntrinsicFunction::ListSet.
        // IMPORTANT: ListSet needs to know if it's setting via the List Header or the Buffer Ptr.
        // Your previous implementation passed `list_ptr_value_id`.
        // If `ListSet` expects the List Struct Ptr, pass `list_header_ptr`.

        for (index, item) in item_values.into_iter().enumerate() {
            let result = self.emit_intrinsic_function_call(
                ctx,
                IntrinsicFunction::ListSet {
                    list_base_ptr: list_header_ptr, // Pass the header pointer
                    index: Value::NumberLiteral(NumberKind::USize(index)),
                    item,
                },
            );
            if let Err(error) = result {
                return Value::Use(self.report_error_and_get_poison(ctx, error));
            }
        }

        // Register the type of the result
        // The result is a POINTER to the List Struct
        let result_type = Type {
            kind: TypeKind::Pointer(Box::new(list_type)),
            span: expr_span,
        };

        // Since emit_heap_alloc already registered the type for list_header_ptr,
        // we might not need to re-register, but it's safe to ensure.

        Value::Use(list_header_ptr)
    }
}
