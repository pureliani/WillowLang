use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
        Span,
    },
    check::SemanticChecker,
};

impl<'a> SemanticChecker<'a> {
    pub fn check_less_than_expr(&mut self, left: Box<Expr>, right: Box<Expr>, span: Span) -> CheckedExpr {
        let checked_left = self.check_expr(*left);
        let checked_right = self.check_expr(*right);
        let checked_op = self.check_binary_numeric_operation(&checked_left, &checked_right, span);

        let expr_type = if matches!(checked_op.kind, CheckedTypeKind::Unknown) {
            checked_op
        } else {
            CheckedType {
                kind: CheckedTypeKind::Bool,
                span,
            }
        };

        CheckedExpr {
            ty: expr_type,
            kind: CheckedExprKind::LessThan {
                left: Box::new(checked_left),
                right: Box::new(checked_right),
            },
        }
    }
}
