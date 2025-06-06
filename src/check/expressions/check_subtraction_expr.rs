use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::checked_expression::{CheckedExpr, CheckedExprKind},
        Span,
    },
    check::{scope::Scope, SemanticChecker},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_subtraction_expr(
        &mut self,
        left: Box<Expr>,
        right: Box<Expr>,
        span: Span,
        scope: Rc<RefCell<Scope>>,
    ) -> CheckedExpr {
        let checked_left = self.check_expr(*left, scope.clone());
        let checked_right = self.check_expr(*right, scope);
        let expr_type = self.check_binary_numeric_operation(&checked_left, &checked_right, span);

        CheckedExpr {
            span,
            ty: expr_type,
            kind: CheckedExprKind::Subtract {
                left: Box::new(checked_left),
                right: Box::new(checked_right),
            },
        }
    }
}
