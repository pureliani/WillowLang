use crate::{
    ast::{
        base::{base_expression::Expr, base_type::TypeAnnotation},
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
        Span,
    },
    check::{SemanticChecker, SemanticError},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_is_type_expr(&mut self, left: Box<Expr>, target: TypeAnnotation, span: Span) -> CheckedExpr {
        let checked_left = self.check_expr(*left);
        let checked_target = self.check_type_annotation(&target);
        let mut expr_type = CheckedType {
            kind: CheckedTypeKind::Bool,
            span,
        };

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
