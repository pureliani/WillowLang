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
    pub fn check_logical_negation_expr(&mut self, right: Box<Expr>, span: Span, scope: Rc<RefCell<Scope>>) -> CheckedExpr {
        let node_id = self.get_node_id();
        self.span_registry.insert_span(node_id, span);

        let checked_right = self.check_expr(*right, scope);

        let expected = CheckedTypeKind::Bool { node_id };
        let mut expr_type = expected.clone();

        if !self.check_is_assignable(&checked_right.ty, &expected) {
            self.errors.push(SemanticError {
                kind: SemanticErrorKind::TypeMismatch {
                    expected,
                    received: checked_right.ty.clone(),
                },
                span,
            });
            expr_type = CheckedTypeKind::Unknown { node_id }
        }

        CheckedExpr {
            span,
            ty: expr_type,
            kind: CheckedExprKind::Not {
                right: Box::new(checked_right),
            },
        }
    }
}
