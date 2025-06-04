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
    check::{scope::Scope, SemanticChecker, SemanticError, SemanticErrorKind},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_and_expr(
        &mut self,
        left: Box<Expr>,
        right: Box<Expr>,
        span: Span,
        scope: Rc<RefCell<Scope>>,
    ) -> CheckedExpr {
        let mut expr_type = CheckedType::Bool;

        let checked_left = self.check_expr(*left, scope.clone());
        let checked_right = self.check_expr(*right, scope);

        if !self.check_is_assignable(&checked_left.ty, &CheckedType::Bool) {
            self.errors.push(SemanticError {
                kind: SemanticErrorKind::TypeMismatch {
                    expected: CheckedType::Bool,
                    received: checked_left.ty.clone(),
                },
                span: checked_left.span,
            });

            expr_type = CheckedType::Unknown;
        }

        if !self.check_is_assignable(&checked_right.ty, &CheckedType::Bool) {
            self.errors.push(SemanticError {
                kind: SemanticErrorKind::TypeMismatch {
                    expected: CheckedType::Bool,
                    received: checked_right.ty.clone(),
                },
                span: checked_right.span,
            });
            expr_type = CheckedType::Unknown;
        }

        CheckedExpr {
            kind: CheckedExprKind::And {
                left: Box::new(checked_left),
                right: Box::new(checked_right),
            },
            span,
            ty: expr_type,
        }
    }
}
