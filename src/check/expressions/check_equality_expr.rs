use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
        Span,
    },
    check::{utils::check_is_equatable::check_is_equatable, SemanticChecker, SemanticError},
    tfg::{TFGNodeId, TFGNodeKind},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_equality_expr(
        &mut self,
        left: Box<Expr>,
        right: Box<Expr>,
        span: Span,
        current_node: TFGNodeId,
        next_node_if_true: TFGNodeId,
        next_node_if_false: TFGNodeId,
    ) -> CheckedExpr {
        let mut type_kind = CheckedTypeKind::Bool;

        let intermediate_node = self.tfg().graph.create_node(TFGNodeKind::NoOp);

        let checked_left = self.check_expr(*left, current_node, intermediate_node, intermediate_node);
        let checked_right = self.check_expr(*right, intermediate_node, next_node_if_true, next_node_if_false);

        if !check_is_equatable(&checked_left.ty.kind, &checked_right.ty.kind) {
            self.errors.push(SemanticError::CannotCompareType {
                of: checked_left.ty.clone(),
                to: checked_right.ty.clone(),
            });

            type_kind = CheckedTypeKind::Unknown
        }

        CheckedExpr {
            ty: CheckedType { kind: type_kind, span },
            kind: CheckedExprKind::Equal {
                left: Box::new(checked_left),
                right: Box::new(checked_right),
            },
        }
    }
}
