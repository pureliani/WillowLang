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
    pub fn check_and_expr(&mut self, left: Box<Expr>, right: Box<Expr>, span: Span) -> CheckedExpr {
        let mut expr_type = Type {
            kind: TypeKind::Bool,
            span,
        };

        let checked_left = self.check_expr(*left);
        let checked_right = self.check_expr(*right);

        let expected_left = Type {
            kind: TypeKind::Bool,
            span: checked_left.ty.span,
        };

        if !self.check_is_assignable(&checked_left.ty, &expected_left) {
            self.errors.push(SemanticError::TypeMismatch {
                expected: expected_left,
                received: checked_left.ty.clone(),
            });

            expr_type.kind = TypeKind::Unknown;
        }

        let expected_right = Type {
            kind: TypeKind::Bool,
            span: checked_right.ty.span,
        };

        if !self.check_is_assignable(&checked_right.ty, &expected_right) {
            self.errors.push(SemanticError::TypeMismatch {
                expected: expected_right,
                received: checked_right.ty.clone(),
            });

            expr_type.kind = TypeKind::Unknown;
        }

        CheckedExpr {
            kind: CheckedExprKind::And {
                left: Box::new(checked_left),
                right: Box::new(checked_right),
            },
            ty: expr_type,
        }
    }
}
