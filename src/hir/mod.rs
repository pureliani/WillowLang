pub mod counters;

use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Weak, sync::Arc};

use crate::{
    ast::{decl::Declaration, stmt::StmtKind, DeclarationId},
    compile::{
        interner::{SharedStringInterner, SharedTagInterner, StringId},
        ParallelParseResult,
    },
    hir::{
        cfg::{
            basic_blocks::BasicBlockId, CheckedModule, ConstantId, ControlFlowGraph,
            ValueId,
        },
        errors::SemanticError,
        statements::{from::build_from_stmt, type_alias_decl::build_type_alias_decl},
        types::{
            checked_declaration::{CheckedDeclaration, CheckedFnDecl, FnType},
            checked_type::Type,
        },
        utils::{
            check_type::{check_params, check_type_annotation},
            scope::{Scope, ScopeKind},
        },
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
    pub function_builder: &'a mut FunctionBuilder,
}

pub struct CommonIdentifiers {
    ptr: StringId,
    capacity: StringId,
    is_heap_allocated: StringId,
    len: StringId,
    id: StringId,
    value: StringId,
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
}

#[derive(Debug)]
pub struct ModuleBuilder {
    pub program_builder: Weak<RefCell<ProgramBuilder>>,

    pub module: CheckedModule,
    /// Module-specific errors
    pub errors: Vec<SemanticError>,
    /// Stack of closures
    pub scopes: Vec<Scope>,
}

#[derive(Debug, Clone)]
pub struct TypePredicate {
    /// The original ValueId that was checked, could be a pointer or a value
    pub source: ValueId,
    /// The new ValueId to use in the true path
    pub true_id: ValueId,
    /// The new ValueId to use in the false path
    pub false_id: ValueId,
}

#[derive(Debug)]
pub struct FunctionBuilder {
    pub parent_module: Weak<RefCell<ModuleBuilder>>,

    pub cfg: ControlFlowGraph,
    pub return_type: Type,
    pub value_definitions: HashMap<ValueId, BasicBlockId>,
    /// Maps a boolean ValueId to the narrowing facts it carries
    pub predicates: HashMap<ValueId, TypePredicate>,
}

impl ProgramBuilder {
    pub fn new(
        string_interner: Arc<SharedStringInterner>,
        tag_interner: Arc<SharedTagInterner>,
    ) -> Self {
        let common_identifiers = CommonIdentifiers {
            id: string_interner.intern("id"),
            value: string_interner.intern("value"),
            capacity: string_interner.intern("capacity"),
            is_heap_allocated: string_interner.intern("is_heap_allocated"),
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
        }
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

    pub fn get_value_type(&self, value_id: &ValueId) -> Type {
        self.value_types
            .get(value_id)
            .expect(
                "INTERNAL COMPILER ERROR: All ValueIds must have a corresponding type",
            )
            .clone()
    }

    // pub fn get_value_type(&self, value: &ValueId) -> Type {
    //     match value {
    //         Value::VoidLiteral => Type::Void,
    //         Value::BoolLiteral(_) => Type::Bool,
    //         Value::NumberLiteral(kind) => match kind {
    //             NumberKind::I64(_) => Type::I64,
    //             NumberKind::I32(_) => Type::I32,
    //             NumberKind::I16(_) => Type::I16,
    //             NumberKind::I8(_) => Type::I8,
    //             NumberKind::F32(_) => Type::F32,
    //             NumberKind::F64(_) => Type::F64,
    //             NumberKind::U64(_) => Type::U64,
    //             NumberKind::U32(_) => Type::U32,
    //             NumberKind::U16(_) => Type::U16,
    //             NumberKind::U8(_) => Type::U8,
    //             NumberKind::USize(_) => Type::USize,
    //             NumberKind::ISize(_) => Type::ISize,
    //         },
    //         Value::Use(value_id) => self.get_value_id_type(value_id),
    //         Value::Function(declaration_id) => {
    //             let fn_decl = self.get_declaration(*declaration_id);
    //             match fn_decl {
    //                 CheckedDeclaration::Function(checked_fn_decl) => Type::Fn(FnType {
    //                     params: checked_fn_decl.params.clone(),
    //                     return_type: Box::new(checked_fn_decl.return_type.clone()),
    //                 }),
    //                 CheckedDeclaration::TypeAlias(..)
    //                 | CheckedDeclaration::Var(..)
    //                 | CheckedDeclaration::UninitializedVar { .. } => todo!(),
    //             }
    //         }
    //     }
    // }

    pub fn build(&mut self, results: Vec<ParallelParseResult>) {
        // Pass 1: Signatures
        for res in &results {
            let mut mb = ModuleBuilder::new(res.path.clone());
            for decl in &res.declarations {
                match decl {
                    Declaration::TypeAlias(d) => {
                        if d.is_exported {
                            mb.module.exports.insert(d.identifier.name);
                        }
                        let mut ctx = HIRContext {
                            program_builder: self,
                            module_builder: &mut mb,
                        };
                        build_type_alias_decl(&mut ctx, d.clone(), d.identifier.span);
                    }
                    Declaration::Fn(d) => {
                        if d.is_exported {
                            mb.module.exports.insert(d.identifier.name);
                        }
                        let mut ctx = HIRContext {
                            program_builder: self,
                            module_builder: &mut mb,
                        };
                        let checked_params = check_params(&mut ctx, &d.params);
                        let return_type = check_type_annotation(&mut ctx, &d.return_type);

                        let checked_fn = CheckedFnDecl {
                            id: d.id,
                            identifier: d.identifier,
                            params: checked_params,
                            return_type,
                            body: None,
                            is_exported: d.is_exported,
                        };

                        ctx.module_builder.scope_insert(
                            ctx.program_builder,
                            d.identifier,
                            CheckedDeclaration::Function(checked_fn),
                        );
                    }
                }
            }
            self.modules.insert(res.path.clone(), mb);
        }

        // Pass 2: Imports
        for res in &results {
            let mut mb = self.modules.remove(&res.path).unwrap();
            {
                let mut ctx = HIRContext {
                    program_builder: self,
                    module_builder: &mut mb,
                };
                for stmt in &res.statements {
                    if let StmtKind::From { path, identifiers } = &stmt.kind {
                        build_from_stmt(
                            &mut ctx,
                            path.clone(),
                            identifiers.clone(),
                            stmt.span,
                        );
                    }
                }
            }
            self.modules.insert(res.path.clone(), mb);
        }

        // Pass 3: Bodies
        for res in &results {
            let mut mb = self.modules.remove(&res.path).unwrap();
            for decl in &res.declarations {
                if let Declaration::Fn(fn_decl) = decl {
                    let mut ctx = HIRContext {
                        program_builder: self,
                        module_builder: &mut mb,
                    };

                    FunctionBuilder::build(&mut ctx, fn_decl.clone());
                }
            }
            self.modules.insert(res.path.clone(), mb);
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
