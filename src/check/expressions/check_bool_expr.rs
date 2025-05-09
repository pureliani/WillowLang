use crate::ast::{
    checked::{
        checked_expression::{CheckedExpr, CheckedExprKind},
        checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
    },
    Span,
};

pub fn check_bool_expr(value: bool, expr_span: Span) -> CheckedExpr {
    CheckedExpr {
        kind: CheckedExprKind::BoolLiteral { value },

        expr_type: CheckedType {
            kind: CheckedTypeKind::Bool,
            span: TypeSpan::Expr(expr_span),
        },
    }
}
