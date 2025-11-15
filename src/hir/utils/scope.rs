use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};

use crate::{
    ast::{IdentifierNode, Span},
    compile::interner::{SharedStringInterner, StringId},
    hir::{
        cfg::{BasicBlockId, CheckedDeclaration},
        errors::{SemanticError, SemanticErrorKind},
        FunctionBuilder, ModuleBuilder,
    },
};

#[derive(Debug)]
pub enum ScopeKind {
    Function(Box<FunctionBuilder>),
    While {
        break_target: BasicBlockId,
        continue_target: BasicBlockId,
    },
    CodeBlock,
    File,
    GenericParams, // Not used for now
}

#[derive(Debug)]
pub struct Scope {
    pub kind: ScopeKind,
    symbols: HashMap<StringId, CheckedDeclaration>,
}

impl Scope {
    pub fn new(kind: ScopeKind) -> Scope {
        Scope {
            symbols: HashMap::new(),
            kind,
        }
    }
}

impl ModuleBuilder {
    pub fn enter_scope(&mut self, kind: ScopeKind) {
        self.scopes.push(Scope::new(kind));
    }

    pub fn exit_scope(&mut self) -> Scope {
        self.scopes
            .pop()
            .expect("INTERNAL COMPILER ERROR: Expected to be able to pop the last scope")
    }

    pub fn last_scope(&self) -> &Scope {
        self.scopes
            .last()
            .expect("INTERNAL COMPILER ERROR: Expected to find the last scope")
    }

    pub fn last_scope_mut(&mut self) -> &mut Scope {
        self.scopes
            .last_mut()
            .expect("INTERNAL COMPILER ERROR: Expected to find the last mutable scope")
    }

    pub fn scope_insert(
        &mut self,
        id: IdentifierNode,
        declaration: CheckedDeclaration,
        span: Span,
    ) {
        let last_scope = self.last_scope_mut();
        if let Entry::Vacant(e) = last_scope.symbols.entry(id.name) {
            e.insert(declaration);
        } else {
            self.errors.push(SemanticError {
                kind: SemanticErrorKind::DuplicateIdentifier(id),
                span,
            });
        }
    }

    pub fn scope_replace(
        &mut self,
        id: IdentifierNode,
        declaration: CheckedDeclaration,
        string_interner: Arc<SharedStringInterner>,
    ) {
        let last_scope = self.last_scope_mut();

        let existing_decl = last_scope
            .symbols
            .get_mut(&id.name)
            .unwrap_or_else(|| {
                let name = string_interner.resolve(id.name);
                panic!(
                    "INTERNAL COMPILER ERROR: Expected to find uninitialized variable '{}' in scope for replacement",
                    name
                )
            });

        if !matches!(existing_decl, CheckedDeclaration::UninitializedVar { .. }) {
            let name = string_interner.resolve(id.name);
            panic!(
                "INTERNAL COMPILER ERROR: Attempted to replace a variable '{}' that was not in an uninitialized state",
                name
            );
        }

        *existing_decl = declaration;
    }

    pub fn scope_lookup(&self, key: StringId) -> Option<&CheckedDeclaration> {
        for scope in self.scopes.iter().rev() {
            if let Some(declaration) = scope.symbols.get(&key) {
                return Some(declaration);
            }
        }
        None
    }

    pub fn scope_lookup_mut(&mut self, key: StringId) -> Option<&mut CheckedDeclaration> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(declaration) = scope.symbols.get_mut(&key) {
                return Some(declaration);
            }
        }
        None
    }

    pub fn within_function_scope(&self) -> bool {
        for scope in self.scopes.iter().rev() {
            if matches!(scope.kind, ScopeKind::Function(_)) {
                return true;
            }
        }

        false
    }

    pub fn within_loop_scope(&self) -> Option<(BasicBlockId, BasicBlockId)> {
        for scope in self.scopes.iter().rev() {
            match scope.kind {
                ScopeKind::CodeBlock => {}
                ScopeKind::While {
                    continue_target,
                    break_target,
                } => return Some((continue_target, break_target)),
                _ => return None,
            }
        }

        None
    }

    pub fn is_file_scope(&self) -> bool {
        matches!(self.last_scope().kind, ScopeKind::File)
    }

    pub fn get_active_function_builder(&mut self) -> &mut FunctionBuilder {
        for scope in self.scopes.iter_mut().rev() {
            if let ScopeKind::Function(builder) = &mut scope.kind {
                return builder;
            }
        }
        panic!("INTERNAL COMPILER ERROR: Expected to find an active function builder on the scope stack.");
    }
}
