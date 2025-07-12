use crate::{
    ast::stmt::Stmt,
    hir_builder::{
        errors::SemanticError,
        utils::scope::{Scope, ScopeKind},
    },
};

pub mod errors;
pub mod expressions;
pub mod module;
pub mod statements;
pub mod types;
pub mod utils;

#[derive(Debug)]
pub struct HIRBuilder<'a> {
    errors: &'a mut Vec<SemanticError>,
    scopes: Vec<Scope>,
    definition_counter: usize,
}

impl<'a> HIRBuilder<'a> {
    pub fn build(statements: Vec<Stmt>) {
        let mut errors: Vec<SemanticError> = vec![];

        let mut builder = HIRBuilder {
            errors: &mut errors,
            definition_counter: 0,
            scopes: vec![Scope::new(ScopeKind::File)],
        };

        let stmts = builder.build_statements(statements);
    }
}
