use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{Type, TypeKind},
        },
        Span,
    },
    check::{SemanticChecker, SemanticError},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_or_expr(&mut self, left: Box<Expr>, right: Box<Expr>, span: Span) -> CheckedExpr {
        let mut ty = Type {
            kind: TypeKind::Bool,
            span,
        };

        let checked_left = self.check_expr(*left);
        let checked_right = self.check_expr(*right);

        let expected_left = Type {
            kind: TypeKind::Bool,
            span: checked_left.ty.span,
        };

        let expected_right = Type {
            kind: TypeKind::Bool,
            span: checked_right.ty.span,
        };

        if !self.check_is_assignable(&checked_left.ty, &expected_left) {
            self.errors.push(SemanticError::TypeMismatch {
                expected: expected_left,
                received: checked_left.ty.clone(),
            });

            ty.kind = TypeKind::Unknown;
        }

        if !self.check_is_assignable(&checked_right.ty, &expected_right) {
            self.errors.push(SemanticError::TypeMismatch {
                expected: expected_right,
                received: checked_right.ty.clone(),
            });

            ty.kind = TypeKind::Unknown;
        }

        CheckedExpr {
            ty,
            kind: CheckedExprKind::Or {
                left: Box::new(checked_left),
                right: Box::new(checked_right),
            },
        }
    }
}
