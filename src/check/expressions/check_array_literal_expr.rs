use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{Type, TypeKind, TypeSpan},
        },
        Span,
    },
    check::{check_expr::check_expr, scope::Scope, utils::union_of::union_of, SemanticError},
};

pub fn check_array_literal_expr(
    items: Vec<Expr>,
    expr_span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let size = items.len();

    let checked_items: Vec<CheckedExpr> = items
        .into_iter()
        .map(|item| check_expr(item, errors, scope.clone()))
        .collect();

    let unionized_types = union_of(
        &checked_items
            .iter()
            .map(|item| item.expr_type.clone())
            .collect::<Vec<Type>>(),
    );

    CheckedExpr {
        expr_type: Type {
            kind: TypeKind::Array {
                item_type: Box::new(unionized_types),
                size,
            },
            span: TypeSpan::Expr(expr_span),
        },
        kind: CheckedExprKind::ArrayLiteral {
            items: checked_items,
        },
    }
}
