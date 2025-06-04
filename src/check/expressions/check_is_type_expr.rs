use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::{base_expression::Expr, base_type::TypeAnnotation},
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::CheckedType,
        },
        Span,
    },
    check::{scope::Scope, SemanticChecker, SemanticError, SemanticErrorKind},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_is_type_expr(
        &mut self,
        left: Box<Expr>,
        target: TypeAnnotation,
        span: Span,
        scope: Rc<RefCell<Scope>>,
    ) -> CheckedExpr {
        let checked_left = self.check_expr(*left, scope.clone());
        let checked_target = self.check_type(&target, scope);

        // TODO: do an actual check
        if !matches!(checked_left.ty, CheckedType::Union { .. }) {
            self.errors.push(SemanticError {
                kind: SemanticErrorKind::CannotUseIsTypeOnNonUnion,
                span,
            });
        }

        CheckedExpr {
            span,
            ty: CheckedType::Bool,
            kind: CheckedExprKind::IsType {
                left: Box::new(checked_left),
                target: checked_target,
            },
        }
    }
}
