use crate::ast::{
    checked::{
        checked_expression::{CheckedExpr, CheckedExprKind},
        checked_type::{Type, TypeKind, TypeSpan},
    },
    Span,
};

pub fn check_bool_expr(value: bool, expr_span: Span) -> CheckedExpr {
    CheckedExpr {
        kind: CheckedExprKind::BoolLiteral { value },

        expr_type: Type {
            kind: TypeKind::Bool,
            span: TypeSpan::Expr(expr_span),
        },
    }
}
