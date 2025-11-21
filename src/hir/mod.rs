use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, RwLock,
    },
};

use crate::{
    ast::{stmt::Stmt, IdentifierNode},
    compile::interner::{SharedStringInterner, SharedTagInterner, StringId},
    hir::{
        cfg::{
            BasicBlock, BasicBlockId, CheckedModule, ConstantId, ControlFlowGraph,
            DeclarationId, FunctionId, HeapAllocationId, Value, ValueId,
        },
        errors::SemanticError,
        types::{
            checked_declaration::{CheckedFnDecl, CheckedParam},
            checked_type::{StructLayout, Type},
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

pub struct CommonIdentifiers {
    env_ptr: StringId,
    fn_ptr: StringId,
    ptr: StringId,
    capacity: StringId,
    len: StringId,
    id: StringId,
    value: StringId,
    payload: StringId,
}

pub struct ProgramBuilder {
    pub modules: HashMap<PathBuf, ModuleBuilder>,
    pub value_types: HashMap<ValueId, Type>,

    pub functions: HashMap<FunctionId, Arc<RwLock<CheckedFnDecl>>>,

    pub string_interner: Arc<SharedStringInterner>,
    pub tag_interner: Arc<SharedTagInterner>,
    pub common_identifiers: CommonIdentifiers,

    // errors and counters
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
    pub fn new(
        string_interner: Arc<SharedStringInterner>,
        tag_interner: Arc<SharedTagInterner>,
    ) -> Self {
        let common_identifiers = CommonIdentifiers {
            id: string_interner.intern("id"),
            value: string_interner.intern("value"),
            payload: string_interner.intern("payload"),
            capacity: string_interner.intern("capacity"),
            env_ptr: string_interner.intern("env_ptr"),
            fn_ptr: string_interner.intern("fn_ptr"),
            len: string_interner.intern("len"),
            ptr: string_interner.intern("ptr"),
        };

        ProgramBuilder {
            errors: vec![],
            modules: HashMap::new(),
            value_types: HashMap::new(),
            string_interner,
            tag_interner,
            common_identifiers,
            functions: HashMap::new(),
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
            Value::VoidLiteral => Type::Void,
            Value::BoolLiteral(_) => Type::Bool,
            Value::NumberLiteral(kind) => {
                let ty = match kind {
                    NumberKind::I64(_) => Type::I64,
                    NumberKind::I32(_) => Type::I32,
                    NumberKind::I16(_) => Type::I16,
                    NumberKind::I8(_) => Type::I8,
                    NumberKind::F32(_) => Type::F32,
                    NumberKind::F64(_) => Type::F64,
                    NumberKind::U64(_) => Type::U64,
                    NumberKind::U32(_) => Type::U32,
                    NumberKind::U16(_) => Type::U16,
                    NumberKind::U8(_) => Type::U8,
                    NumberKind::USize(_) => Type::USize,
                    NumberKind::ISize(_) => Type::ISize,
                };

                ty
            }
            Value::StringLiteral(_) => Type::Struct(StructLayout::const_string(self)),
            Value::Function { ty, .. } => ty.clone(),
            Value::Use(value_id) => self.get_value_id_type(value_id),
        }
    }
}

impl ModuleBuilder {
    pub fn new(path: PathBuf) -> Self {
        Self {
            module: CheckedModule::new(path),
            errors: vec![],
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
