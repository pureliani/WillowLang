use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedTypeX, CheckedType, TypeSpan},
        },
        Span,
    },
    check::{
        check_expr::check_expr,
        scope::Scope,
        utils::{get_numeric_type_rank::get_numeric_type_rank, is_integer::is_integer},
        SemanticError, SemanticErrorKind,
    },
};

pub fn check_equality_expr(
    left: Box<Expr>,
    right: Box<Expr>,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let span = Span {
        start: left.span.start,
        end: right.span.end,
    };
    let mut expr_type = CheckedTypeX {
        kind: CheckedType::Bool,
        span: TypeSpan::Expr(span.clone()),
    };

    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_right = check_expr(*right, errors, scope);

    let err = SemanticError::new(
        SemanticErrorKind::CannotCompareType {
            of: checked_left.ty.clone(),
            to: checked_right.ty.clone(),
        },
        span,
    );

    if !is_integer(&checked_left.ty)
        || !is_integer(&checked_right.ty)
        || get_numeric_type_rank(&checked_left.ty)
            < get_numeric_type_rank(&checked_right.ty)
    {
        errors.push(err);
        expr_type.kind = CheckedType::Unknown
    } else {
        match (&checked_left.ty.kind, &checked_right.ty.kind) {
            (CheckedType::Bool, CheckedType::Bool) => {}
            (CheckedType::Char, CheckedType::Char) => {}
            (CheckedType::Null, CheckedType::Null) => {}
            (CheckedType::Enum(_), CheckedType::Enum(_)) => {}
            _ => {
                errors.push(err);
                expr_type.kind = CheckedType::Unknown
            }
        }
    }

    CheckedExpr {
        kind: CheckedExprKind::Equal {
            left: Box::new(checked_left),
            right: Box::new(checked_right),
        },
        ty: expr_type,
    }
}
