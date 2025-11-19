use crate::{
    ast::{expr::Expr, IdentifierNode, Span},
    hir::{
        cfg::Value,
        types::{
            checked_declaration::CheckedTagType,
            checked_type::{Type, TypeKind},
        },
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_tag_expr(
        &mut self,
        ctx: &mut HIRContext,
        name: IdentifierNode,
        value: Option<Box<Expr>>,
        span: Span,
    ) -> Value {
        let tag_id = ctx.program_builder.tag_interner.intern(&name.name);
        let inner_value = value.map(|v| Box::new(self.build_expr(ctx, *v)));
        let inner_value_type = inner_value
            .as_ref()
            .map(|v| Box::new(ctx.program_builder.get_value_type(v)));

        let checked_type = Type {
            kind: TypeKind::Tag(CheckedTagType {
                span,
                identifier: name,
                value_type: inner_value_type,
            }),
            span,
        };

        Value::Tag {
            tag_id,
            value: inner_value,
            ty: checked_type,
        }
    }
}
