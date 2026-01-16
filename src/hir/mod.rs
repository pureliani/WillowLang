use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use crate::{
    compile::interner::{SharedStringInterner, SharedTagInterner, StringId},
    hir::{
        cfg::{
            BasicBlockId, CheckedModule, ConstantId, ControlFlowGraph, DeclarationId,
            Value, ValueId,
        },
        errors::SemanticError,
        types::{
            checked_declaration::{CheckedDeclaration, FnType},
            checked_type::Type,
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
    pub refinements: HashMap<(BasicBlockId, ValueId), Type>,
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
            Value::NumberLiteral(kind) => match kind {
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
            },
            Value::Use(value_id) => self.get_value_id_type(value_id),
            Value::Function(declaration_id) => {
                let fn_decl = self.get_declaration(*declaration_id);
                match fn_decl {
                    CheckedDeclaration::Function(checked_fn_decl) => Type::Fn(FnType {
                        params: checked_fn_decl.params.clone(),
                        return_type: Box::new(checked_fn_decl.return_type.clone()),
                    }),
                    CheckedDeclaration::TypeAlias(..)
                    | CheckedDeclaration::Var(..)
                    | CheckedDeclaration::UninitializedVar { .. } => todo!(),
                }
            }
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
}
