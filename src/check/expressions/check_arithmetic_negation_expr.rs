use std::collections::HashSet;

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
        Span,
    },
    check::{utils::is_signed::is_signed, SemanticChecker, SemanticError},
    tfg::{TFGNodeId, TFGNodeKind},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_arithmetic_negation_expr(
        &mut self,
        right: Box<Expr>,
        span: Span,
        current_node: TFGNodeId,
        next_node_if_true: TFGNodeId,
        next_node_if_false: TFGNodeId,
    ) -> CheckedExpr {
        let tfg = self.tfg();
        let op_node = tfg.graph.create_node(TFGNodeKind::NoOp);

        tfg.graph.link(op_node, next_node_if_true);
        tfg.graph.link(op_node, next_node_if_false);

        let checked_right = self.check_expr(*right, current_node, op_node, op_node);

        let expr_type = match &checked_right.ty.kind {
            t if is_signed(&t) => CheckedType { kind: t.clone(), span },
            _ => {
                let expected = HashSet::from([
                    CheckedType {
                        kind: CheckedTypeKind::I8,
                        span: checked_right.ty.span,
                    },
                    CheckedType {
                        kind: CheckedTypeKind::I16,
                        span: checked_right.ty.span,
                    },
                    CheckedType {
                        kind: CheckedTypeKind::I32,
                        span: checked_right.ty.span,
                    },
                    CheckedType {
                        kind: CheckedTypeKind::I64,
                        span: checked_right.ty.span,
                    },
                    CheckedType {
                        kind: CheckedTypeKind::ISize,
                        span: checked_right.ty.span,
                    },
                    CheckedType {
                        kind: CheckedTypeKind::F32,
                        span: checked_right.ty.span,
                    },
                    CheckedType {
                        kind: CheckedTypeKind::F64,
                        span: checked_right.ty.span,
                    },
                ]);

                self.errors.push(SemanticError::TypeMismatch {
                    expected: CheckedType {
                        kind: CheckedTypeKind::Union(expected),
                        span: checked_right.ty.span,
                    },
                    received: checked_right.ty.clone(),
                });

                CheckedType {
                    kind: CheckedTypeKind::Unknown,
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
