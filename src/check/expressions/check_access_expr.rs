use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_declaration::CheckedStructDecl,
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::CheckedType,
        },
        IdentifierNode, Span,
    },
    check::{check_expr::check_expr, scope::Scope, SemanticError, SemanticErrorKind},
};

pub fn check_access_expr(
    left: Box<Expr>,
    field: IdentifierNode,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let span = Span {
        start: left.span.start,
        end: field.span.end,
    };
    let checked_left = check_expr(*left, errors, scope);

    let expr_type = match &checked_left.ty {
        CheckedType::StructDecl(CheckedStructDecl { properties, .. }) => properties
            .into_iter()
            .find(|p| p.identifier == field)
            .map(|p| p.constraint.clone())
            .unwrap_or_else(|| {
                errors.push(SemanticError::new(
                    SemanticErrorKind::UndefinedProperty(field.clone()),
                    span,
                ));
                CheckedType::Unknown
            }),
        t => {
            errors.push(SemanticError::new(
                SemanticErrorKind::CannotAccess(t.clone()),
                field.span,
            ));

            CheckedType::Unknown
        }
    };

    CheckedExpr {
        kind: CheckedExprKind::Access {
            left: Box::new(checked_left.clone()),
            field,
        },
        span,
        ty: expr_type,
    }
}
