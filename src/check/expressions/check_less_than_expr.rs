use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::CheckedTypeKind,
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
        let node_id = self.get_node_id();
        self.span_registry.insert_span(node_id, span);

        let checked_left = self.check_expr(*left, scope.clone());
        let checked_right = self.check_expr(*right, scope);
        let checked_op = self.check_binary_numeric_operation(&checked_left, &checked_right, span);

        let expr_type = if matches!(checked_op, CheckedTypeKind::Unknown { .. }) {
            checked_op
        } else {
            CheckedTypeKind::Bool { node_id }
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
