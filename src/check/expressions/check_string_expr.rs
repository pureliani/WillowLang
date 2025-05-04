use crate::ast::{
    checked::{
        checked_expression::{CheckedExpr, CheckedExprKind},
        checked_type::{Type, TypeKind, TypeSpan},
    },
    Span, StringNode,
};

pub fn check_string_expr(string_node: StringNode, expr_span: Span) -> CheckedExpr {
    CheckedExpr {
        expr_type: Type {
            kind: TypeKind::Array {
                item_type: Box::new(Type {
                    kind: TypeKind::Char,
                    span: TypeSpan::None,
                }),
                size: string_node.value.len(),
            },
            span: TypeSpan::Expr(expr_span),
        },
        kind: CheckedExprKind::String(string_node),
    }
}
