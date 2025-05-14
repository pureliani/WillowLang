use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::ast::{
    base::base_declaration::EnumDecl,
    checked::checked_declaration::{
        CheckedGenericParam, CheckedGenericStructDecl, CheckedGenericTypeAliasDecl,
        CheckedStructDecl, CheckedTypeAliasDecl, CheckedVarDecl,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub enum ScopeKind {
    Function,
    While,
    CodeBlock,
    File,
    TypeAlias,
    Struct,
    FnType,
}

#[derive(Debug, Clone)]
pub enum SymbolEntry {
    GenericStructDecl(CheckedGenericStructDecl),
    StructDecl(CheckedStructDecl),
    EnumDecl(EnumDecl),
    VarDecl(CheckedVarDecl),
    GenericTypeAliasDecl(CheckedGenericTypeAliasDecl),
    TypeAliasDecl(CheckedTypeAliasDecl),
    GenericParam(CheckedGenericParam),
}

#[derive(Debug, Clone)]
pub struct Scope {
    parent: Option<Rc<RefCell<Scope>>>,
    symbols: HashMap<String, SymbolEntry>,
    pub kind: ScopeKind,
}

impl Scope {
    pub fn new(kind: ScopeKind) -> Scope {
        Scope {
            parent: None,
            symbols: HashMap::new(),
            kind,
        }
    }

    pub fn new_with_parent(parent: Rc<RefCell<Scope>>, kind: ScopeKind) -> Scope {
        Scope {
            parent: Some(parent),
            symbols: HashMap::new(),
            kind,
        }
    }

    pub fn insert(&mut self, key: String, value: SymbolEntry) {
        self.symbols.insert(key, value);
    }

    pub fn lookup(&self, key: &str) -> Option<SymbolEntry> {
        if let Some(value) = self.symbols.get(key) {
            return Some(value.clone());
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().lookup(key);
        }

        None
    }

    pub fn is_within_function(&self) -> bool {
        if self.kind != ScopeKind::Function {
            self.parent
                .as_ref()
                .map(|p| p.borrow().is_within_function())
                .unwrap_or(false)
        } else {
            true
        }
    }

    pub fn is_within_loop(&self) -> bool {
        if self.kind != ScopeKind::While {
            self.parent
                .as_ref()
                .map(|p| {
                    let p = p.borrow();
                    if p.kind != ScopeKind::Function && p.kind != ScopeKind::File {
                        p.is_within_loop()
                    } else {
                        false
                    }
                })
                .unwrap_or(false)
        } else {
            true
        }
    }

    pub fn child(&self, kind: ScopeKind) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Scope::new_with_parent(
            Rc::new(RefCell::new(self.clone())),
            kind,
        )))
    }
}
