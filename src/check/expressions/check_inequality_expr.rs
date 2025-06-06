use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
        Span,
    },
    check::{scope::Scope, utils::check_is_equatable::check_is_equatable, SemanticChecker, SemanticError},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_inequality_expr(
        &mut self,
        left: Box<Expr>,
        right: Box<Expr>,
        span: Span,
        scope: Rc<RefCell<Scope>>,
    ) -> CheckedExpr {
        let mut expr_type = CheckedType {
            kind: CheckedTypeKind::Bool,
            span,
        };

        let checked_left = self.check_expr(*left, scope.clone());
        let checked_right = self.check_expr(*right, scope);

        if !check_is_equatable(&checked_left.ty.kind, &checked_right.ty.kind) {
            self.errors.push(SemanticError::CannotCompareType {
                of: checked_left.ty.clone(),
                to: checked_right.ty.clone(),
            });

            expr_type.kind = CheckedTypeKind::Unknown;
        }

        CheckedExpr {
            ty: expr_type,
            kind: CheckedExprKind::NotEqual {
                left: Box::new(checked_left),
                right: Box::new(checked_right),
            },
        }
    }
}
