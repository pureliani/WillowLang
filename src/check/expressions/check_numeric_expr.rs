use crate::{
    ast::{
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
        },
        Span,
    },
    tokenizer::NumberKind,
};

pub fn check_numeric_expr(value: NumberKind, expr_span: Span) -> CheckedExpr {
    let type_kind = match value {
        NumberKind::I64(_) => CheckedTypeKind::I64,
        NumberKind::I32(_) => CheckedTypeKind::I32,
        NumberKind::I16(_) => CheckedTypeKind::I16,
        NumberKind::I8(_) => CheckedTypeKind::I8,
        NumberKind::F32(_) => CheckedTypeKind::F32,
        NumberKind::F64(_) => CheckedTypeKind::F64,
        NumberKind::U64(_) => CheckedTypeKind::U64,
        NumberKind::U32(_) => CheckedTypeKind::U32,
        NumberKind::U16(_) => CheckedTypeKind::U16,
        NumberKind::U8(_) => CheckedTypeKind::U8,
        NumberKind::USize(_) => CheckedTypeKind::USize,
        NumberKind::ISize(_) => CheckedTypeKind::ISize,
    };

    CheckedExpr {
        kind: CheckedExprKind::Number { value },
        expr_type: CheckedType {
            kind: type_kind,
            span: TypeSpan::Expr(expr_span),
        },
    }
}
