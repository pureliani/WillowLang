use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::{base_expression::Expr, base_type::TypeAnnotation},
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
        Span,
    },
    check::{scope::Scope, SemanticChecker, SemanticError},
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
        let mut expr_type = CheckedType {
            kind: CheckedTypeKind::Bool,
            span,
        };

        // TODO: do an actual check
        if !matches!(checked_left.ty.kind, CheckedTypeKind::Union(..)) {
            self.errors.push(SemanticError::CannotUseIsTypeOnNonUnion {
                target: checked_left.ty.clone(),
            });

            expr_type.kind = CheckedTypeKind::Unknown;
        }

        CheckedExpr {
            ty: expr_type,
            kind: CheckedExprKind::IsType {
                left: Box::new(checked_left),
                target: checked_target,
            },
        }
    }
}
