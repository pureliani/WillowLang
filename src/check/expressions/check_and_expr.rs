use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
        Span,
    },
    check::{SemanticChecker, SemanticError},
    tfg::{TFGNodeId, TFGNodeKind},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_and_expr(
        &mut self,
        left: Box<Expr>,
        right: Box<Expr>,
        span: Span,
        current_node: TFGNodeId,
        next_node_if_true: TFGNodeId,
        next_node_if_false: TFGNodeId,
    ) -> CheckedExpr {
        let mut expr_type = CheckedType {
            kind: CheckedTypeKind::Bool,
            span,
        };

        let intermediate_node = self.tfg().graph.create_node(TFGNodeKind::NoOp);

        let checked_left = self.check_expr(*left, current_node, intermediate_node, next_node_if_false);
        let checked_right = self.check_expr(*right, intermediate_node, next_node_if_true, next_node_if_false);

        let expected_left = CheckedType {
            kind: CheckedTypeKind::Bool,
            span: checked_left.ty.span,
        };

        if !self.check_is_assignable(&checked_left.ty, &expected_left) {
            self.errors.push(SemanticError::TypeMismatch {
                expected: expected_left,
                received: checked_left.ty.clone(),
            });

            expr_type.kind = CheckedTypeKind::Unknown;
        }

        let expected_right = CheckedType {
            kind: CheckedTypeKind::Bool,
            span: checked_right.ty.span,
        };

        if !self.check_is_assignable(&checked_right.ty, &expected_right) {
            self.errors.push(SemanticError::TypeMismatch {
                expected: expected_right,
                received: checked_right.ty.clone(),
            });

            expr_type.kind = CheckedTypeKind::Unknown;
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
