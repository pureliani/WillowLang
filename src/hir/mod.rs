use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};

use crate::{
    ast::{stmt::Stmt, IdentifierNode},
    compile::string_interner::{SharedStringInterner, StringInterner},
    hir::{
        cfg::{
            BasicBlock, BasicBlockId, CheckedDeclaration, CheckedModule, ConstantId,
            ControlFlowGraph, DeclarationId, FunctionId, HeapAllocationId, Value,
            ValueId,
        },
        errors::SemanticError,
        types::{
            checked_declaration::CheckedParam,
            checked_type::{Type, TypeKind},
        },
        utils::scope::{Scope, ScopeKind},
    },
    tokenize::NumberKind,
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

pub struct HIRContext<'a> {
    pub program_builder: &'a mut ProgramBuilder,
    pub module_builder: &'a mut ModuleBuilder,
}

pub struct ProgramBuilder {
    pub modules: HashMap<PathBuf, ModuleBuilder>,
    pub declarations: HashMap<DeclarationId, CheckedDeclaration>,
    pub value_types: HashMap<ValueId, Type>,
    pub string_interner: Arc<SharedStringInterner>,
    /// Global errors
    pub errors: Vec<SemanticError>,

    value_id_counter: AtomicUsize,
    function_id_counter: AtomicUsize,
    constant_id_counter: AtomicUsize,
    allocation_id_counter: AtomicUsize,
    declaration_id_counter: AtomicUsize,
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

impl ProgramBuilder {
    pub fn new(string_interner: Arc<SharedStringInterner>) -> Self {
        ProgramBuilder {
            errors: vec![],
            modules: HashMap::new(),
            value_types: HashMap::new(),
            declarations: HashMap::new(),
            string_interner,
            function_id_counter: AtomicUsize::new(0),
            constant_id_counter: AtomicUsize::new(0),
            allocation_id_counter: AtomicUsize::new(0),
            value_id_counter: AtomicUsize::new(0),
            declaration_id_counter: AtomicUsize::new(0),
        }
    }

    pub fn build_module(&mut self, path: PathBuf, statements: Vec<Stmt>) {
        let mut module_builder = ModuleBuilder::new(path.clone());
        module_builder.build_top_level_statements(self, statements);
        self.modules.insert(path, module_builder);
    }

    pub fn finish(self) -> (HashMap<PathBuf, ModuleBuilder>, Vec<SemanticError>) {
        let mut global_errors = vec![];

        // TODO: Check all imports were resolved.
        // TODO: Check for a single `main` function in the whole program.

        (self.modules, global_errors)
    }

    pub fn new_declaration_id(&self) -> DeclarationId {
        DeclarationId(self.declaration_id_counter.fetch_add(1, Ordering::SeqCst))
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

    pub fn new_value_id(&self) -> ValueId {
        ValueId(self.value_id_counter.fetch_add(1, Ordering::SeqCst))
    }

    pub fn get_value_id_type(&self, value_id: &ValueId) -> Type {
        self.value_types
            .get(value_id)
            .expect(
                "INTERNAL COMPILER ERROR: All ValueIds must have a corresponding type",
            )
            .clone()
    }

    pub fn get_value_type(&self, value: &Value) -> Type {
        match value {
            Value::VoidLiteral => Type {
                kind: TypeKind::Void,
                span: Default::default(),
            },
            Value::BoolLiteral(_) => Type {
                kind: TypeKind::Bool,
                // TODO: fix this later
                span: Default::default(),
            },
            Value::NumberLiteral(kind) => {
                let kind = match kind {
                    NumberKind::I64(_) => TypeKind::I64,
                    NumberKind::I32(_) => TypeKind::I32,
                    NumberKind::I16(_) => TypeKind::I16,
                    NumberKind::I8(_) => TypeKind::I8,
                    NumberKind::F32(_) => TypeKind::F32,
                    NumberKind::F64(_) => TypeKind::F64,
                    NumberKind::U64(_) => TypeKind::U64,
                    NumberKind::U32(_) => TypeKind::U32,
                    NumberKind::U16(_) => TypeKind::U16,
                    NumberKind::U8(_) => TypeKind::U8,
                    NumberKind::USize(_) => TypeKind::USize,
                    NumberKind::ISize(_) => TypeKind::ISize,
                };

                Type {
                    kind,
                    span: Default::default(),
                }
            }
            Value::StringLiteral(_) => Type {
                kind: TypeKind::String,
                span: Default::default(),
            },
            Value::FunctionAddr { ty, .. } => ty.clone(),
            Value::Use(value_id) => self.get_value_id_type(value_id),
        }
    }
}

impl ModuleBuilder {
    pub fn new(path: PathBuf) -> Self {
        Self {
            module: CheckedModule::new(path),
            errors: vec![],
            functions: vec![],
            scopes: vec![Scope::new(ScopeKind::File)],
        }
    }

    fn build_top_level_statements(
        &mut self,
        program_builder: &mut ProgramBuilder,
        statements: Vec<Stmt>,
    ) {
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
                    terminator: None,
                },
            )]),
            entry_block: entry_block_id,
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

    pub fn build_body(
        &mut self,
        ctx: &mut HIRContext,
        params: Vec<CheckedParam>,
        body: Vec<Stmt>,
    ) {
        for param in params {
            todo!()
        }

        for stmt in body {
            todo!()
        }
    }
}
