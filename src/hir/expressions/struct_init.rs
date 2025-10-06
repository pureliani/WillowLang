use std::collections::{HashMap, HashSet};

use crate::{
    ast::{expr::Expr, IdentifierNode, Span},
    hir::{
        cfg::Value,
        errors::{SemanticError, SemanticErrorKind},
        types::checked_type::{Type, TypeKind},
        FunctionBuilder, HIRContext,
    },
    tokenize::NumberKind,
};

pub fn get_alignment_of(type_kind: &TypeKind) -> usize {
    use std::mem::align_of;

    match type_kind {
        TypeKind::U64 | TypeKind::I64 | TypeKind::F64 => 8,
        TypeKind::U32 | TypeKind::I32 | TypeKind::F32 => 4,
        TypeKind::U16 | TypeKind::I16 => 2,
        TypeKind::U8 | TypeKind::I8 | TypeKind::Bool => 1,
        TypeKind::Pointer(_) | TypeKind::USize | TypeKind::ISize | TypeKind::FnType(_) => align_of::<usize>(),
        TypeKind::Struct(_) | TypeKind::String => align_of::<usize>(),
        TypeKind::Enum(_) => align_of::<isize>(),
        TypeKind::Void => 1,
        TypeKind::Unknown => 1,
        TypeKind::TypeAliasDecl(decl) => get_alignment_of(&decl.value.kind),
    }
}

impl FunctionBuilder {
    pub fn build_struct_init_expr(&mut self, ctx: &mut HIRContext, fields: Vec<(IdentifierNode, Expr)>, span: Span) -> Value {
        let mut resolved_fields: Vec<(IdentifierNode, Type)> = Vec::with_capacity(fields.len());
        let mut field_values: HashMap<IdentifierNode, Value> = HashMap::with_capacity(fields.len());
        let mut initialized_fields: HashSet<IdentifierNode> = HashSet::new();

        for (field_name, field_expr) in fields {
            if !initialized_fields.insert(field_name) {
                return Value::Use(self.report_error_and_get_poison(
                    ctx,
                    SemanticError {
                        kind: SemanticErrorKind::DuplicateStructFieldInitializer(field_name),
                        span: field_name.span,
                    },
                ));
            }

            let value = self.build_expr(ctx, field_expr);
            let value_type = ctx.program_builder.get_value_type(&value);

            resolved_fields.push((field_name, value_type));
            field_values.insert(field_name, value);
        }

        resolved_fields.sort_by(|(identifier_a, type_a), (identifier_b, type_b)| {
            let align_a = get_alignment_of(&type_a.kind);
            let align_b = get_alignment_of(&type_b.kind);

            align_b.cmp(&align_a).then_with(|| {
                let name_a = ctx.program_builder.string_interner.resolve(identifier_a.name);
                let name_b = ctx.program_builder.string_interner.resolve(identifier_b.name);

                name_a.cmp(name_b)
            })
        });

        let struct_type = Type {
            kind: TypeKind::Struct(resolved_fields),
            span,
        };

        let struct_ptr = self
            .emit_heap_alloc(ctx, struct_type.clone(), Value::NumberLiteral(NumberKind::USize(1)))
            .expect("INTERNAL COMPILER ERROR: failed to allocate struct on heap");

        if let TypeKind::Struct(canonical_fields) = &struct_type.kind {
            for (field_name, _) in canonical_fields {
                let field_ptr = match self.emit_get_field_ptr(ctx, struct_ptr, *field_name) {
                    Ok(ptr) => ptr,
                    Err(error) => return Value::Use(self.report_error_and_get_poison(ctx, error)),
                };

                let field_value = field_values.get(field_name).unwrap();

                self.emit_store(ctx, field_ptr, field_value.clone());
            }
        }

        Value::Use(struct_ptr)
    }
}
