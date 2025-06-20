use crate::{
    ast::{
        base::base_expression::Expr,
        checked::checked_expression::{CheckedExpr, CheckedExprKind},
        Span,
    },
    check::SemanticChecker,
};

impl<'a> SemanticChecker<'a> {
    pub fn check_division_expr(&mut self, left: Box<Expr>, right: Box<Expr>, span: Span) -> CheckedExpr {
        let checked_left = self.check_expr(*left);
        let checked_right = self.check_expr(*right);
        let expr_type = self.check_binary_numeric_operation(&checked_left, &checked_right, span);

        CheckedExpr {
            kind: CheckedExprKind::Divide {
                left: Box::new(checked_left),
                right: Box::new(checked_right),
            },
            ty: expr_type,
        }
    }
}
