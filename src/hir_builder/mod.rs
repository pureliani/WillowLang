use std::{
    collections::{HashMap, HashSet},
    vec,
};

use crate::{
    ast::{stmt::Stmt, IdentifierNode},
    cfg::{BasicBlockId, CheckedModule, ControlFlowGraph, ModuleId, ValueId},
    compile::string_interner::StringInterner,
    hir_builder::{
        errors::{SemanticError, SemanticErrorKind},
        types::{checked_declaration::CheckedParam, checked_type::Type},
        utils::scope::{Scope, ScopeKind},
    },
};

pub mod errors;
pub mod expressions;
pub mod statements;
pub mod types;
pub mod utils;

#[derive(Debug, Clone)]
pub struct CapturedVar {
    pub original_value_id: ValueId,
    pub captured_as_field_ptr_id: ValueId,
    pub identifier: IdentifierNode,
}

#[derive(Debug)]
pub struct FunctionBuilder {
    pub cfg: ControlFlowGraph,
    pub return_type: Type,
    pub params: Vec<CheckedParam>,
    pub captures: HashSet<CapturedVar>,

    pub current_block_id: BasicBlockId,
    block_id_counter: usize,
    value_id_counter: usize,
    allocation_id_counter: usize,
}

pub struct ModuleBuilder<'a> {
    pub module: CheckedModule,
    pub errors: Vec<SemanticError>,
    pub string_interner: &'a StringInterner<'a>,
    pub function_builders: Vec<FunctionBuilder>,
    pub scopes: Vec<Scope>,

    function_id_counter: usize,
    constant_id_counter: usize,
}

impl<'a> ModuleBuilder<'a> {
    pub fn build(
        module_id: ModuleId,
        name: String,
        string_interner: &'a StringInterner<'a>,
    ) -> (CheckedModule, Vec<SemanticError>) {
        let module = CheckedModule {
            id: module_id,
            name,
            constant_data: HashMap::new(),
            declarations: HashMap::new(),
            exports: HashSet::new(),
            functions: HashMap::new(),
        };

        let mut builder = ModuleBuilder {
            module,
            scopes: vec![Scope::new(ScopeKind::File)],
            errors: vec![],
            string_interner,
            function_builders: vec![],
            function_id_counter: 0,
            constant_id_counter: 0,
        };

        builder.build_top_level_statements();

        (builder.module, builder.errors)
    }

    pub fn build_top_level_statements(&mut self) {
        todo!()
    }

    pub fn current_function_builder(&mut self) -> &mut FunctionBuilder {
        self.function_builders
            .last_mut()
            .expect("INTERNAL: Builder stack should never be empty.")
    }
}

impl FunctionBuilder {
    pub fn build(
        module_builder: &mut ModuleBuilder,
        params: Vec<CheckedParam>,
        expected_return_type: Type,
        body: Vec<Stmt>,
    ) -> (ControlFlowGraph, HashSet<CapturedVar>) {
        let cfg = ControlFlowGraph {
            blocks: HashMap::new(),
            entry_block: BasicBlockId(0),
            value_types: HashMap::new(),
        };

        let mut builder = FunctionBuilder {
            cfg,
            params,
            return_type: expected_return_type,
            captures: HashSet::new(),
            block_id_counter: 0,
            current_block_id: BasicBlockId(0),
            value_id_counter: 0,
            allocation_id_counter: 0,
        };

        builder.build_statements(module_builder, body);

        (builder.cfg, builder.captures)
    }
}
