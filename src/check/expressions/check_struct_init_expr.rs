use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_declaration::{CheckedGenericStructDecl, CheckedStructDecl},
            checked_expression::CheckedExpr,
            checked_type::CheckedType,
        },
        IdentifierNode,
    },
    check::{check_expr::check_expr, scope::Scope, SemanticError},
};

pub fn check_struct_init_expr(
    left: Box<Expr>,
    fields: Vec<(IdentifierNode, Expr)>,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_fields: Vec<(IdentifierNode, CheckedExpr)> = fields
        .into_iter()
        .map(|f| (f.0, check_expr(f.1, errors, scope.clone())))
        .collect();

    match checked_left.ty.kind {
        CheckedType::GenericStructDecl(CheckedGenericStructDecl {
            identifier,
            properties,
            documentation,
            generic_params,
        }) => {
            todo!()
        }
        CheckedType::StructDecl(CheckedStructDecl {
            identifier,
            properties,
            documentation,
        }) => {
            todo!()
        }
        _ => {
            todo!()
        }
    }

    todo!()
}
