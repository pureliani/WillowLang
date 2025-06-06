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
    check::{scope::Scope, SemanticChecker, SemanticError, SemanticErrorKind},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_or_expr(&mut self, left: Box<Expr>, right: Box<Expr>, span: Span, scope: Rc<RefCell<Scope>>) -> CheckedExpr {
        let node_id = self.get_node_id();
        self.span_registry.insert_span(node_id, span);

        let expected = CheckedTypeKind::Bool { node_id };
        let mut ty = expected.clone();

        let checked_left = self.check_expr(*left, scope.clone());
        let checked_right = self.check_expr(*right, scope);

        if !self.check_is_assignable(&checked_left.ty, &CheckedTypeKind::Bool { node_id }) {
            self.errors.push(SemanticError {
                kind: SemanticErrorKind::TypeMismatch {
                    expected: expected.clone(),
                    received: checked_left.ty.clone(),
                },
                span: checked_left.span,
            });
            ty = CheckedTypeKind::Unknown { node_id };
        }

        if !self.check_is_assignable(&checked_right.ty, &CheckedTypeKind::Bool { node_id }) {
            self.errors.push(SemanticError {
                kind: SemanticErrorKind::TypeMismatch {
                    expected,
                    received: checked_right.ty.clone(),
                },
                span: checked_right.span,
            });
            ty = CheckedTypeKind::Unknown { node_id };
        }

        CheckedExpr {
            span,
            kind: CheckedExprKind::Or {
                left: Box::new(checked_left),
                right: Box::new(checked_right),
            },
            ty,
        }
    }
}
