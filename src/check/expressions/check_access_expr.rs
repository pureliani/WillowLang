use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_declaration::StructDecl,
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
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

    let expr_type = match &checked_left.expr_type.kind {
        CheckedTypeKind::StructDecl(StructDecl { properties, .. }) => properties
            .iter()
            .find(|p| p.identifier == field)
            .map(|p| p.constraint.clone())
            .unwrap_or(CheckedType {
                kind: CheckedTypeKind::Unknown,
                span: TypeSpan::Expr(field.span),
            }),
        _ => {
            errors.push(SemanticError::new(
                SemanticErrorKind::UndefinedProperty(field.clone()),
                field.span,
            ));

            CheckedType {
                kind: CheckedTypeKind::Unknown,
                span: TypeSpan::Expr(span),
            }
        }
    };

    CheckedExpr {
        kind: CheckedExprKind::Access {
            left: Box::new(checked_left.clone()),
            field,
        },
        expr_type,
    }
}
