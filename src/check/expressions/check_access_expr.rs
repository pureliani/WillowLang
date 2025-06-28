use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
        IdentifierNode, Span,
    },
    check::{SemanticChecker, SemanticError},
    tfg::{TFGNodeId, TFGNodeKind},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_access_expr(
        &mut self,
        left: Box<Expr>,
        field: IdentifierNode,
        span: Span,
        current_node: TFGNodeId,
        next_node_if_true: TFGNodeId,
        next_node_if_false: TFGNodeId,
    ) -> CheckedExpr {
        let tfg = self.tfg();
        let access_node = tfg.graph.create_node(TFGNodeKind::NoOp);

        tfg.graph.link(access_node, next_node_if_true);
        tfg.graph.link(access_node, next_node_if_false);

        let checked_left = self.check_expr(*left, current_node, access_node, access_node);

        let expr_type = match &checked_left.ty.kind {
            // TODO: Add enum declaration handler
            CheckedTypeKind::StructDecl(decl) => decl
                .borrow()
                .fields
                .iter()
                .find(|p| p.identifier == field)
                .map(|p| p.constraint.clone())
                .unwrap_or_else(|| {
                    self.errors.push(SemanticError::AccessToUndefinedField { field });

                    CheckedType {
                        kind: CheckedTypeKind::Unknown,
                        span,
                    }
                }),
            _ => {
                self.errors.push(SemanticError::CannotAccess {
                    target: checked_left.ty.clone(),
                });

                CheckedType {
                    kind: CheckedTypeKind::Unknown,
                    span,
                }
            }
        };

        CheckedExpr {
            ty: expr_type,
            kind: CheckedExprKind::Access {
                left: Box::new(checked_left.clone()),
                field,
            },
        }
    }
}
