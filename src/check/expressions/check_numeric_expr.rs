use crate::{
    ast::{
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{Type, TypeKind, TypeSpan},
        },
        Span,
    },
    tokenizer::NumberKind,
};

pub fn check_numeric_expr(value: NumberKind, expr_span: Span) -> CheckedExpr {
    let type_kind = match value {
        NumberKind::I64(_) => TypeKind::I64,
        NumberKind::I32(_) => TypeKind::I32,
        NumberKind::I16(_) => TypeKind::I16,
        NumberKind::I8(_) => TypeKind::I8,
        NumberKind::F32(_) => TypeKind::F32,
        NumberKind::F64(_) => TypeKind::F64,
        NumberKind::U64(_) => TypeKind::U64,
        NumberKind::U32(_) => TypeKind::U32,
        NumberKind::U16(_) => TypeKind::U16,
        NumberKind::U8(_) => TypeKind::U8,
        NumberKind::USize(_) => TypeKind::USize,
        NumberKind::ISize(_) => TypeKind::ISize,
    };

    CheckedExpr {
        kind: CheckedExprKind::Number { value },
        expr_type: Type {
            kind: type_kind,
            span: TypeSpan::Expr(expr_span),
        },
    }
}
