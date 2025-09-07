use std::collections::HashMap;

use crate::{
    ast::{IdentifierNode, Span},
    cfg::BasicBlockId,
    compile::string_interner::InternerId,
    hir_builder::{
        errors::SemanticError,
        types::checked_declaration::{CheckedStructDecl, CheckedTypeAliasDecl, CheckedUnionDecl, CheckedVarDecl},
        HIRBuilder, SemanticErrorKind,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub enum ScopeKind {
    Function,
    While {
        break_target: BasicBlockId,
        continue_target: BasicBlockId,
    },
    CodeBlock,
    File,
    TypeAlias,
    FnType,
}

#[derive(Debug, Clone)]
pub enum SymbolEntry {
    VarDecl(CheckedVarDecl),
    TypeAliasDecl(CheckedTypeAliasDecl),
    StructDecl(CheckedStructDecl),
    UnionDecl(CheckedUnionDecl),
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

impl<'a> HIRBuilder<'a> {
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

    pub fn scope_insert(&mut self, id: IdentifierNode, value: SymbolEntry, span: Span) {
        let last_scope = self.last_scope_mut();

        if let Some(_) = last_scope.symbols.insert(id.name, value) {
            self.errors.push(SemanticError {
                kind: SemanticErrorKind::DuplicateIdentifier(id),
                span,
            });
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
        self.last_scope().kind == ScopeKind::File
    }
}
