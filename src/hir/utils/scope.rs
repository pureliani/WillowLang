use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};

use crate::{
    ast::IdentifierNode,
    compile::interner::{SharedStringInterner, StringId},
    hir::{
        cfg::{BasicBlockId, DeclarationId},
        errors::{SemanticError, SemanticErrorKind},
        types::checked_declaration::CheckedDeclaration,
        ModuleBuilder, ProgramBuilder,
    },
};

#[derive(Debug)]
pub enum ScopeKind {
    Function,
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
    symbols: HashMap<StringId, DeclarationId>,
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
        program_builder: &mut ProgramBuilder,
        id: IdentifierNode,
        declaration: CheckedDeclaration,
    ) {
        let decl_id = match &declaration {
            CheckedDeclaration::Var(decl) => decl.id,
            CheckedDeclaration::TypeAlias(decl) => decl.id,
            CheckedDeclaration::Function(decl) => decl.id,
            CheckedDeclaration::UninitializedVar { id, .. } => *id,
        };

        program_builder.declarations.insert(decl_id, declaration);

        let last_scope = self.last_scope_mut();
        if let Entry::Vacant(e) = last_scope.symbols.entry(id.name) {
            e.insert(decl_id);
        } else {
            self.errors.push(SemanticError {
                kind: SemanticErrorKind::DuplicateIdentifier(id),
                span: id.span,
            });
        }
    }

    pub fn scope_replace(
        &mut self,
        program_builder: &mut ProgramBuilder,
        id: IdentifierNode,
        new_declaration: CheckedDeclaration,
        string_interner: Arc<SharedStringInterner>,
    ) {
        let last_scope = self.last_scope_mut();

        let existing_decl_id = last_scope.symbols.get(&id.name).unwrap_or_else(|| {
            let name = string_interner.resolve(id.name);
            panic!(
                "INTERNAL COMPILER ERROR: Expected to find uninitialized variable '{}' \
                 in scope for replacement",
                name
            )
        });

        let existing_decl = program_builder.get_declaration_mut(*existing_decl_id);
        if !matches!(
            existing_decl,
            &mut CheckedDeclaration::UninitializedVar { .. }
        ) {
            let name = string_interner.resolve(id.name);
            panic!(
                "INTERNAL COMPILER ERROR: Attempted to replace a variable '{}' that was \
                 not in an uninitialized state",
                name
            );
        }

        if !matches!(&new_declaration, &CheckedDeclaration::Var { .. }) {
            let name = string_interner.resolve(id.name);
            panic!(
                "INTERNAL COMPILER ERROR: Attempted to replace an uninitialized \
                 variable '{}' with something other than initialized variable",
                name
            );
        }

        *existing_decl = new_declaration;
    }

    pub fn scope_lookup(&self, key: StringId) -> Option<DeclarationId> {
        for scope in self.scopes.iter().rev() {
            if let Some(id) = scope.symbols.get(&key) {
                return Some(*id);
            }
        }
        None
    }

    pub fn within_function_scope(&self) -> bool {
        for scope in self.scopes.iter().rev() {
            if matches!(scope.kind, ScopeKind::Function) {
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

    pub fn scope_map(&mut self, id: IdentifierNode, decl_id: DeclarationId) {
        let last_scope = self.last_scope_mut();

        if let Entry::Vacant(e) = last_scope.symbols.entry(id.name) {
            e.insert(decl_id);
        } else {
            self.errors.push(SemanticError {
                kind: SemanticErrorKind::DuplicateIdentifier(id),
                span: id.span,
            });
        }
    }

    pub fn resolve_export(&self, name: StringId) -> Option<DeclarationId> {
        if !self.module.exports.contains(&name) {
            return None;
        }

        self.scopes
            .first()
            .and_then(|s| s.symbols.get(&name).copied())
    }
}
