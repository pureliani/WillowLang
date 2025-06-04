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
    check::{
        scope::Scope, utils::check_is_equatable::check_is_equatable, SemanticChecker,
        SemanticError, SemanticErrorKind,
    },
};

impl<'a> SemanticChecker<'a> {
    pub fn check_equality_expr(
        &mut self,
        left: Box<Expr>,
        right: Box<Expr>,
        span: Span,
        scope: Rc<RefCell<Scope>>,
    ) -> CheckedExpr {
        let mut ty = CheckedType::Bool;

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

            ty = CheckedType::Unknown
        }

        CheckedExpr {
            ty,
            span,
            kind: CheckedExprKind::Equal {
                left: Box::new(checked_left),
                right: Box::new(checked_right),
            },
        }
    }
}
