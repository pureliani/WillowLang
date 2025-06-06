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
    check::{scope::Scope, utils::check_is_equatable::check_is_equatable, SemanticChecker, SemanticError, SemanticErrorKind},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_inequality_expr(
        &mut self,
        left: Box<Expr>,
        right: Box<Expr>,
        span: Span,
        scope: Rc<RefCell<Scope>>,
    ) -> CheckedExpr {
        let node_id = self.get_node_id();
        self.span_registry.insert_span(node_id, span);

        let mut ty = CheckedTypeKind::Bool { node_id };

        let checked_left = self.check_expr(*left, scope.clone());
        let checked_right = self.check_expr(*right, scope);

        if !check_is_equatable(&checked_left.ty, &checked_right.ty) {
            self.errors.push(SemanticError {
                kind: SemanticErrorKind::CannotCompareType {
                    of: checked_left.ty.clone(),
                    to: checked_right.ty.clone(),
                },
                span,
            });

            ty = CheckedTypeKind::Unknown { node_id };
        }

        CheckedExpr {
            ty,
            span,
            kind: CheckedExprKind::NotEqual {
                left: Box::new(checked_left),
                right: Box::new(checked_right),
            },
        }
    }
}
