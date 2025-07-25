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
    pub fn check_logical_negation_expr(&mut self, right: Box<Expr>, span: Span) -> CheckedExpr {
        let checked_right = self.check_expr(*right);

        let expected_right = Type {
            kind: TypeKind::Bool,
            span: checked_right.ty.span,
        };

        let mut expr_type = Type {
            kind: TypeKind::Bool,
            span,
        };

        if !self.check_is_assignable(&checked_right.ty, &expected_right) {
            self.errors.push(SemanticError::TypeMismatch {
                expected: expected_right,
                received: checked_right.ty.clone(),
            });

            expr_type.kind = TypeKind::Unknown
        }

        CheckedExpr {
            ty: expr_type,
            kind: CheckedExprKind::Not {
                right: Box::new(checked_right),
            },
        }
    }
}
