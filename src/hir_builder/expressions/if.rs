use crate::{
    ast::expr::{BlockContents, Expr},
    cfg::{Terminator, Value},
    hir_builder::{
        errors::{SemanticError, SemanticErrorKind},
        types::checked_type::{Type, TypeKind},
        HIRBuilder,
    },
};

impl<'a> HIRBuilder<'a> {
    pub fn build_if_expr(&mut self, branches: Vec<(Box<Expr>, BlockContents)>, else_branch: Option<BlockContents>) -> Value {
        todo!()
    }
}
