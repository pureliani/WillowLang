use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use crate::{
    ast::stmt::Stmt,
    compile::interner::{SharedStringInterner, SharedTagInterner, StringId},
    hir::{
        cfg::{
            BasicBlock, BasicBlockId, CheckedModule, ConstantId, ControlFlowGraph,
            DeclarationId, Value, ValueId,
        },
        errors::SemanticError,
        types::{checked_declaration::CheckedDeclaration, checked_type::Type},
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

pub struct HIRContext<'a> {
    pub program_builder: &'a mut ProgramBuilder,
    pub module_builder: &'a mut ModuleBuilder,
}

pub struct CommonIdentifiers {
    env_ptr: StringId,
    fn_ptr: StringId,
    ptr: StringId,
    capacity: StringId,
    is_heap_allocated: StringId,
    len: StringId,
    id: StringId,
    value: StringId,
    payload: StringId,
}

pub struct ProgramBuilder {
    pub modules: HashMap<PathBuf, ModuleBuilder>,
    pub value_types: HashMap<ValueId, Type>,

    pub declarations: HashMap<DeclarationId, CheckedDeclaration>,
    pub constant_data: HashMap<ConstantId, Vec<u8>>,

    pub string_interner: Arc<SharedStringInterner>,
    pub tag_interner: Arc<SharedTagInterner>,
    pub common_identifiers: CommonIdentifiers,

    pub errors: Vec<SemanticError>,
    value_id_counter: AtomicUsize,
    constant_id_counter: AtomicUsize,
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
    pub current_block_id: BasicBlockId,

    pub predecessors: HashMap<BasicBlockId, Vec<BasicBlockId>>,
    pub block_value_maps: HashMap<BasicBlockId, HashMap<ValueId, ValueId>>,
    pub value_definitions: HashMap<ValueId, BasicBlockId>,
    pub sealed_blocks: HashSet<BasicBlockId>,
    // Map: BlockId -> List of (PlaceholderParamId, OriginalValueId)
    pub incomplete_params: HashMap<BasicBlockId, Vec<(ValueId, ValueId)>>,

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
            is_heap_allocated: string_interner.intern("is_heap_allocated"),
            env_ptr: string_interner.intern("env_ptr"),
            fn_ptr: string_interner.intern("fn_ptr"),
            len: string_interner.intern("len"),
            ptr: string_interner.intern("ptr"),
        };

        ProgramBuilder {
            errors: vec![],
            modules: HashMap::new(),
            value_types: HashMap::new(),
            constant_data: HashMap::new(),
            string_interner,
            tag_interner,
            common_identifiers,
            declarations: HashMap::new(),
            constant_id_counter: AtomicUsize::new(0),
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

    pub fn get_declaration(&self, id: DeclarationId) -> &CheckedDeclaration {
        self.declarations
            .get(&id)
            .expect("INTERNAL COMPILER ERROR: DeclarationId not found")
    }

    pub fn get_declaration_mut(&mut self, id: DeclarationId) -> &mut CheckedDeclaration {
        self.declarations
            .get_mut(&id)
            .expect("INTERNAL COMPILER ERROR: DeclarationId not found")
    }

    pub fn new_constant_id(&self) -> ConstantId {
        ConstantId(self.constant_id_counter.fetch_add(1, Ordering::SeqCst))
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
                    params: vec![],
                },
            )]),
            entry_block: entry_block_id,
        };

        let mut builder = Self {
            cfg,
            return_type,
            block_value_maps: HashMap::new(),
            incomplete_params: HashMap::new(),
            predecessors: HashMap::new(),
            value_definitions: HashMap::new(),
            sealed_blocks: HashSet::new(),
            current_block_id: entry_block_id,
            block_id_counter: 1,
            value_id_counter: 0,
        };

        builder.sealed_blocks.insert(entry_block_id);

        builder
    }
}
