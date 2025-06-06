use crate::{
    ast::{
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
        Span,
    },
    check::SemanticChecker,
    tokenize::NumberKind,
};

impl<'a> SemanticChecker<'a> {
    pub fn check_numeric_expr(&mut self, value: NumberKind, span: Span) -> CheckedExpr {
        let ty = match value {
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
            ty: CheckedType { kind: ty, span },
            kind: CheckedExprKind::Number { value },
        }
    }
}
