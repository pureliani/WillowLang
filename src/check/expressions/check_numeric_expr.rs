use crate::{
    ast::{
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeX, TypeSpan},
        },
        Span,
    },
    tokenizer::NumberKind,
};

pub fn check_numeric_expr(value: NumberKind, expr_span: Span) -> CheckedExpr {
    let type_kind = match value {
        NumberKind::I64(_) => CheckedType::I64,
        NumberKind::I32(_) => CheckedType::I32,
        NumberKind::I16(_) => CheckedType::I16,
        NumberKind::I8(_) => CheckedType::I8,
        NumberKind::F32(_) => CheckedType::F32,
        NumberKind::F64(_) => CheckedType::F64,
        NumberKind::U64(_) => CheckedType::U64,
        NumberKind::U32(_) => CheckedType::U32,
        NumberKind::U16(_) => CheckedType::U16,
        NumberKind::U8(_) => CheckedType::U8,
        NumberKind::USize(_) => CheckedType::USize,
        NumberKind::ISize(_) => CheckedType::ISize,
    };

    CheckedExpr {
        span: expr_span,
        kind: CheckedExprKind::Number { value },
        ty: CheckedTypeX {
            kind: type_kind,
            span: TypeSpan::Expr(expr_span),
        },
    }
}
