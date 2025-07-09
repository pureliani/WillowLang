use crate::{
    ast::{
        base::base_expression::Expr,
        checked::checked_expression::{CheckedExpr, CheckedExprKind},
        Span,
    },
    check::{
        utils::{get_numeric_type_rank::get_numeric_type_rank, is_float::is_float, is_integer::is_integer, is_signed::is_signed},
        SemanticChecker,
    },
};

use crate::{
    ast::checked::checked_type::{Type, TypeKind},
    check::SemanticError,
};

impl<'a> SemanticChecker<'a> {
    pub fn check_binary_numeric_operation(&mut self, left: &CheckedExpr, right: &CheckedExpr, span: Span) -> Type {
        let left_type = if is_float(&left.ty.kind) || is_integer(&left.ty.kind) {
            &left.ty
        } else {
            self.errors
                .push(SemanticError::ExpectedANumericOperand { span: left.ty.span });

            return Type {
                kind: TypeKind::Unknown,
                span: left.ty.span,
            };
        };

        let right_type = if is_float(&right.ty.kind) || is_integer(&right.ty.kind) {
            &right.ty
        } else {
            self.errors
                .push(SemanticError::ExpectedANumericOperand { span: right.ty.span });

            return Type {
                kind: TypeKind::Unknown,
                span: right.ty.span,
            };
        };

        if (is_float(&left_type.kind) && is_integer(&right_type.kind))
            || (is_integer(&left_type.kind) && is_float(&right_type.kind))
        {
            self.errors.push(SemanticError::MixedFloatAndInteger { span });

            return Type {
                kind: TypeKind::Unknown,
                span: span,
            };
        }

        if is_signed(&left_type.kind) != is_signed(&right_type.kind) {
            self.errors.push(SemanticError::MixedSignedAndUnsigned { span });

            return Type {
                kind: TypeKind::Unknown,
                span: span,
            };
        }

        if right_type == left_type {
            return left_type.clone();
        }

        let left_rank = get_numeric_type_rank(&left_type.kind);
        let right_rank = get_numeric_type_rank(&right_type.kind);

        if left_rank > right_rank {
            Type {
                kind: left_type.kind.clone(),
                span,
            }
        } else {
            Type {
                kind: right_type.kind.clone(),
                span,
            }
        }
    }

    pub fn check_arithmetic_operation(
        &mut self,
        left: Box<Expr>,
        right: Box<Expr>,
        span: Span,
        constructor: fn(Box<CheckedExpr>, Box<CheckedExpr>) -> CheckedExprKind,
    ) -> CheckedExpr {
        let checked_left = self.check_expr(*left);
        let checked_right = self.check_expr(*right);
        let expr_type = self.check_binary_numeric_operation(&checked_left, &checked_right, span);

        CheckedExpr {
            ty: expr_type,
            kind: constructor(Box::new(checked_left), Box::new(checked_right)),
        }
    }

    pub fn check_numeric_comparison(
        &mut self,
        left: Box<Expr>,
        right: Box<Expr>,
        span: Span,
        constructor: fn(Box<CheckedExpr>, Box<CheckedExpr>) -> CheckedExprKind,
    ) -> CheckedExpr {
        let checked_left = self.check_expr(*left);
        let checked_right = self.check_expr(*right);
        let checked_op = self.check_binary_numeric_operation(&checked_left, &checked_right, span);

        let expr_type = if matches!(checked_op.kind, TypeKind::Unknown) {
            checked_op
        } else {
            Type {
                kind: TypeKind::Bool,
                span,
            }
        };

        CheckedExpr {
            ty: expr_type,
            kind: constructor(Box::new(checked_left), Box::new(checked_right)),
        }
    }
}
