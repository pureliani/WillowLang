use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{
        checked::checked_declaration::{CheckedGenericParam, CheckedTypeAliasDecl, CheckedVarDecl},
        IdentifierNode,
    },
    check::{SemanticChecker, SemanticError},
    compile::string_interner::InternerId,
};

#[derive(Debug, Clone, PartialEq)]
pub enum ScopeKind {
    Function,
    While,
    CodeBlock,
    File,
    TypeAlias,
    FnType,
}

#[derive(Debug, Clone)]
pub enum SymbolEntry {
    VarDecl(Rc<RefCell<CheckedVarDecl>>),
    TypeAliasDecl(Rc<RefCell<CheckedTypeAliasDecl>>),
    GenericParam(CheckedGenericParam),
}

#[derive(Debug)]
pub struct Scope {
    pub kind: ScopeKind,
    symbols: HashMap<InternerId, SymbolEntry>,
}

impl Scope {
    pub fn new(kind: ScopeKind) -> Scope {
        Scope {
            symbols: HashMap::new(),
            kind,
        }
    }
}

impl<'a> SemanticChecker<'a> {
    pub fn enter_scope(&mut self, kind: ScopeKind) {
        self.scopes.push(Scope::new(kind));
    }

    pub fn exit_scope(&mut self) -> Scope {
        self.scopes.pop().expect("Expected to be able to pop the last scope")
    }

    pub fn last_scope(&self) -> &Scope {
        self.scopes.last().expect("Expected to find the last scope")
    }

    pub fn last_scope_mut(&mut self) -> &mut Scope {
        self.scopes.last_mut().expect("Expected to find the last mutable scope")
    }

    pub fn scope_insert(&mut self, key: IdentifierNode, value: SymbolEntry) {
        let last_scope = self.last_scope_mut();

        if let Some(_) = last_scope.symbols.insert(key.name, value) {
            self.errors.push(SemanticError::DuplicateIdentifier { id: key });
        }
    }

    pub fn scope_lookup(&self, key: InternerId) -> Option<SymbolEntry> {
        for scope in self.scopes.iter().rev() {
            if let Some(declaration) = scope.symbols.get(&key) {
                let cloned = declaration.to_owned();
                return Some(cloned);
            }
        }

        None
    }

    pub fn within_function_scope(&self) -> bool {
        for scope in self.scopes.iter().rev() {
            if scope.kind == ScopeKind::Function {
                return true;
            }
        }

        return false;
    }

    pub fn within_loop_scope(&self) -> bool {
        for scope in self.scopes.iter().rev() {
            match scope.kind {
                ScopeKind::CodeBlock => {}
                ScopeKind::While => return true,
                _ => return false,
            }
        }

        false
    }

    pub fn is_file_scope(&self) -> bool {
        self.last_scope().kind == ScopeKind::File
    }
}
