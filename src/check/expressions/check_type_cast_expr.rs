use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::{base_expression::Expr, base_type::TypeAnnotation},
        checked::checked_expression::CheckedExpr,
    },
    check::{scope::Scope, SemanticError},
};

pub fn check_type_cast_expr(
    left: Box<Expr>,
    target: TypeAnnotation,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    todo!()
}
