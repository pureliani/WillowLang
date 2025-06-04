use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::CheckedType,
        },
        Span,
    },
    check::{scope::Scope, SemanticChecker},
};
impl<'a> SemanticChecker<'a> {
    pub fn check_less_than_expr(
        &mut self,
        left: Box<Expr>,
        right: Box<Expr>,
        span: Span,
        scope: Rc<RefCell<Scope>>,
    ) -> CheckedExpr {
        let checked_left = self.check_expr(*left, scope.clone());
        let checked_right = self.check_expr(*right, scope);
        let checked_op = self.check_binary_numeric_operation(&checked_left, &checked_right);

        let expr_type = if checked_op == CheckedType::Unknown {
            CheckedType::Unknown
        } else {
            CheckedType::Bool
        };

        CheckedExpr {
            span,
            ty: expr_type,
            kind: CheckedExprKind::LessThan {
                left: Box::new(checked_left),
                right: Box::new(checked_right),
            },
        }
    }
}
