use std::collections::HashMap;

use crate::{
    ast::stmt::Stmt,
    cfg::{BasicBlockId, ControlFlowGraph},
    compile::string_interner::StringInterner,
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
    string_interner: &'a StringInterner<'a>,
    cfg: ControlFlowGraph,
    errors: Vec<SemanticError>,
    scopes: Vec<Scope>,
    current_block_id: BasicBlockId,
    block_id_counter: usize,
    value_id_counter: usize,
}

impl<'a> HIRBuilder<'a> {
    pub fn build(statements: Vec<Stmt>, string_interner: &'a StringInterner<'a>) {
        let cfg = ControlFlowGraph {
            blocks: HashMap::new(),
            entry_block: BasicBlockId(0),
            value_types: HashMap::new(),
        };

        let mut builder = HIRBuilder {
            string_interner,
            cfg,
            errors: vec![],
            scopes: vec![Scope::new(ScopeKind::File)],
            current_block_id: BasicBlockId(0),
            block_id_counter: 0,
            value_id_counter: 0,
        };

        let stmts = builder.build_statements(statements);
    }
}
