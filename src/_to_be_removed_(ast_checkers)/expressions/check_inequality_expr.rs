use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{Type, TypeKind},
        },
        Span,
    },
    check::{utils::check_is_equatable::check_is_equatable, SemanticChecker, SemanticError},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_inequality_expr(&mut self, left: Box<Expr>, right: Box<Expr>, span: Span) -> CheckedExpr {
        let mut expr_type = Type {
            kind: TypeKind::Bool,
            span,
        };

        let checked_left = self.check_expr(*left);
        let checked_right = self.check_expr(*right);

        if !check_is_equatable(&checked_left.ty.kind, &checked_right.ty.kind) {
            self.errors.push(SemanticError::CannotCompareType {
                of: checked_left.ty.clone(),
                to: checked_right.ty.clone(),
            });

            expr_type.kind = TypeKind::Unknown;
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
