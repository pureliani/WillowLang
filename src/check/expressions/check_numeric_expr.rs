use crate::{
    ast::{
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::CheckedTypeKind,
        },
        Span,
    },
    check::SemanticChecker,
    tokenize::NumberKind,
};

impl<'a> SemanticChecker<'a> {
    pub fn check_numeric_expr(&mut self, value: NumberKind, span: Span) -> CheckedExpr {
        let node_id = self.get_node_id();
        self.span_registry.insert_span(node_id, span);

        let ty = match value {
            NumberKind::I64(_) => CheckedTypeKind::I64 { node_id },
            NumberKind::I32(_) => CheckedTypeKind::I32 { node_id },
            NumberKind::I16(_) => CheckedTypeKind::I16 { node_id },
            NumberKind::I8(_) => CheckedTypeKind::I8 { node_id },
            NumberKind::F32(_) => CheckedTypeKind::F32 { node_id },
            NumberKind::F64(_) => CheckedTypeKind::F64 { node_id },
            NumberKind::U64(_) => CheckedTypeKind::U64 { node_id },
            NumberKind::U32(_) => CheckedTypeKind::U32 { node_id },
            NumberKind::U16(_) => CheckedTypeKind::U16 { node_id },
            NumberKind::U8(_) => CheckedTypeKind::U8 { node_id },
            NumberKind::USize(_) => CheckedTypeKind::USize { node_id },
            NumberKind::ISize(_) => CheckedTypeKind::ISize { node_id },
        };

        CheckedExpr {
            ty,
            span,
            kind: CheckedExprKind::Number { value },
        }
    }
}
