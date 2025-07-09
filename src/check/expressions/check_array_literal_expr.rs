use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{Type, TypeKind},
        },
        Span,
    },
    check::{utils::union_of::union_of, SemanticChecker},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_array_literal_expr(&mut self, items: Vec<Expr>, span: Span) -> CheckedExpr {
        let size = items.len();

        let checked_items: Vec<CheckedExpr> = items.into_iter().map(|item| self.check_expr(item)).collect();

        let unionized_types = union_of(checked_items.iter().map(|item| item.ty.clone()), span);

        let expr_type = Type {
            kind: TypeKind::Array {
                item_type: Box::new(unionized_types),
                size,
            },
            span,
        };

        CheckedExpr {
            ty: expr_type,
            kind: CheckedExprKind::ArrayLiteral { items: checked_items },
        }
    }
}
