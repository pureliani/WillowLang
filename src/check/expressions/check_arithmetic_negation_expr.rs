use std::collections::HashSet;

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{Type, TypeKind},
        },
        Span,
    },
    check::{utils::is_signed::is_signed, SemanticChecker, SemanticError},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_arithmetic_negation_expr(&mut self, right: Box<Expr>, span: Span) -> CheckedExpr {
        let checked_right = self.check_expr(*right);

        let expr_type = match &checked_right.ty.kind {
            t if is_signed(&t) => Type { kind: t.clone(), span },
            _ => {
                let expected = HashSet::from([
                    Type {
                        kind: TypeKind::I8,
                        span: checked_right.ty.span,
                    },
                    Type {
                        kind: TypeKind::I16,
                        span: checked_right.ty.span,
                    },
                    Type {
                        kind: TypeKind::I32,
                        span: checked_right.ty.span,
                    },
                    Type {
                        kind: TypeKind::I64,
                        span: checked_right.ty.span,
                    },
                    Type {
                        kind: TypeKind::ISize,
                        span: checked_right.ty.span,
                    },
                    Type {
                        kind: TypeKind::F32,
                        span: checked_right.ty.span,
                    },
                    Type {
                        kind: TypeKind::F64,
                        span: checked_right.ty.span,
                    },
                ]);

                self.errors.push(SemanticError::TypeMismatch {
                    expected: Type {
                        kind: TypeKind::Union(expected),
                        span: checked_right.ty.span,
                    },
                    received: checked_right.ty.clone(),
                });

                Type {
                    kind: TypeKind::Unknown,
                    span,
                }
            }
        };

        CheckedExpr {
            ty: expr_type,
            kind: CheckedExprKind::Neg {
                right: Box::new(checked_right),
            },
        }
    }
}
