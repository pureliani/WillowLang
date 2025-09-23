use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::{
    ast::{stmt::Stmt, IdentifierNode},
    compile::string_interner::StringInterner,
    hir::{
        cfg::{
            BasicBlock, BasicBlockId, CheckedModule, ConstantId, ControlFlowGraph, FunctionId, HeapAllocationId, ModuleId,
            Terminator, ValueId,
        },
        errors::SemanticError,
        types::{checked_declaration::CheckedParam, checked_type::Type},
        utils::scope::{Scope, ScopeKind},
    },
};

pub mod cfg;
pub mod errors;
pub mod expressions;
pub mod statements;
pub mod types;
pub mod utils;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapturedVar {
    pub original_value_id: ValueId,
    pub captured_as_field_ptr_id: ValueId,
    pub identifier: IdentifierNode,
}

pub struct HIRContext<'a, 'b> {
    // A mutable reference to the global program builder for services like ID generation.
    pub program_builder: &'b mut ProgramBuilder<'a>,
    // A mutable reference to the module builder for module-specific state.
    pub module_builder: &'b mut ModuleBuilder,
}

pub struct ProgramBuilder<'a> {
    pub modules: HashMap<ModuleId, ModuleBuilder>,
    pub string_interner: &'a mut StringInterner<'a>,
    /// Global errors
    pub errors: Vec<SemanticError>,

    function_id_counter: AtomicUsize,
    constant_id_counter: AtomicUsize,
    allocation_id_counter: AtomicUsize,
}

#[derive(Debug)]
pub struct ModuleBuilder {
    pub module: CheckedModule,
    /// Module-specific errors
    pub errors: Vec<SemanticError>,
    /// Stack of closures
    pub functions: Vec<FunctionBuilder>,
    pub scopes: Vec<Scope>,
}

#[derive(Debug)]
pub struct FunctionBuilder {
    pub cfg: ControlFlowGraph,
    pub return_type: Type,
    pub captures: HashSet<CapturedVar>,

    pub current_block_id: BasicBlockId,
    block_id_counter: usize,
    value_id_counter: usize,
}

impl<'a> ProgramBuilder<'a> {
    pub fn new(string_interner: &'a mut StringInterner<'a>) -> Self {
        ProgramBuilder {
            errors: vec![],
            modules: HashMap::new(),
            string_interner,
            function_id_counter: AtomicUsize::new(0),
            constant_id_counter: AtomicUsize::new(0),
            allocation_id_counter: AtomicUsize::new(0),
        }
    }

    pub fn build_module(&mut self, module_id: ModuleId, path: PathBuf, statements: Vec<Stmt>) {
        let mut module_builder = ModuleBuilder::new(module_id, path);
        module_builder.build_top_level_statements(self, statements);
        self.modules.insert(module_id, module_builder);
    }

    pub fn finish(self) -> (HashMap<ModuleId, ModuleBuilder>, Vec<SemanticError>) {
        let mut global_errors = vec![];

        // TODO: Check all imports were resolved.
        // TODO: Check for a single `main` function in the whole program.

        (self.modules, global_errors)
    }

    pub fn new_function_id(&self) -> FunctionId {
        FunctionId(self.function_id_counter.fetch_add(1, Ordering::SeqCst))
    }

    pub fn new_constant_id(&self) -> ConstantId {
        ConstantId(self.constant_id_counter.fetch_add(1, Ordering::SeqCst))
    }

    pub fn new_allocation_id(&self) -> HeapAllocationId {
        HeapAllocationId(self.allocation_id_counter.fetch_add(1, Ordering::SeqCst))
    }
}

impl ModuleBuilder {
    pub fn new(id: ModuleId, path: PathBuf) -> Self {
        Self {
            module: CheckedModule::new(id, path),
            errors: vec![],
            functions: vec![],
            scopes: vec![Scope::new(ScopeKind::File)],
        }
    }

    fn build_top_level_statements(&mut self, program_builder: &mut ProgramBuilder, statements: Vec<Stmt>) {
        let mut ctx = HIRContext {
            module_builder: self,
            program_builder,
        };

        // TODO: One pass to add declarations to the scope (handle forward declarations)
        for stmt in &statements {
            todo!()
        }

        // TODO: Generate HIR
        for stmt in &statements {
            todo!()
        }

        todo!()
    }
}

impl FunctionBuilder {
    pub fn new(return_type: Type) -> Self {
        let entry_block_id = BasicBlockId(0);
        let cfg = ControlFlowGraph {
            blocks: HashMap::from([(
                entry_block_id,
                BasicBlock {
                    id: entry_block_id,
                    instructions: vec![],
                    terminator: Terminator::Unreachable,
                },
            )]),
            entry_block: entry_block_id,
            value_types: HashMap::new(),
        };

        Self {
            cfg,
            return_type,
            captures: HashSet::new(),
            current_block_id: entry_block_id,
            block_id_counter: 1,
            value_id_counter: 0,
        }
    }

    pub fn build_body(&mut self, ctx: &mut HIRContext, params: Vec<CheckedParam>, body: Vec<Stmt>) {
        for param in params {
            todo!()
        }

        for stmt in body {
            todo!()
        }
    }
}
