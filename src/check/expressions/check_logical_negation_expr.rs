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
    pub fn check_logical_negation_expr(
        &mut self,
        right: Box<Expr>,
        span: Span,
        scope: Rc<RefCell<Scope>>,
    ) -> CheckedExpr {
        let checked_right = self.check_expr(*right, scope);

        let mut expr_type = CheckedType::Bool;

        if !self.check_is_assignable(&checked_right.ty, &CheckedType::Bool) {
            self.errors.push(SemanticError {
                kind: SemanticErrorKind::TypeMismatch {
                    expected: CheckedType::Bool,
                    received: checked_right.ty.clone(),
                },
                span,
            });
            expr_type = CheckedType::Unknown
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
