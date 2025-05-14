use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::CheckedType,
        },
        Span,
    },
    check::{check_expr::check_expr, scope::Scope, utils::union_of::union_of, SemanticError},
};

pub fn check_array_literal_expr(
    items: Vec<Expr>,
    span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let size = items.len();

    let checked_items: Vec<CheckedExpr> = items
        .into_iter()
        .map(|item| check_expr(item, errors, scope.clone()))
        .collect();

    let unionized_types = union_of(checked_items.iter().map(|item| item.ty.clone()));

    let expr_type = CheckedType::Array {
        item_type: Box::new(unionized_types),
        size,
    };

    CheckedExpr {
        span,
        ty: expr_type,
        kind: CheckedExprKind::ArrayLiteral {
            items: checked_items,
        },
    }
}
