use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::{base_expression::Expr, base_type::TypeAnnotation},
        checked::checked_expression::CheckedExpr,
        Span,
    },
    check::{scope::Scope, SemanticError},
    compile::SpanRegistry,
};
impl<'a> SemanticChecker<'a> {}

pub fn check_type_cast_expr(
    left: Box<Expr>,
    target: TypeAnnotation,
    span: Span,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    todo!()
}
