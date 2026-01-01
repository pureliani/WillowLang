use std::collections::{HashMap, HashSet};

use crate::{
    ast::{decl::FnDecl, expr::BlockContents, visitor::ASTVisitor, IdentifierNode},
    hir::{
        types::{
            checked_declaration::{CheckedDeclaration, CheckedParam},
            checked_type::Type,
        },
        HIRContext,
    },
};

struct CaptureAnalyzerVisitor<'a, 'b> {
    ctx: &'a mut HIRContext<'b>,
    /// Variables declared within the current function scope
    local_scope: HashSet<IdentifierNode>,
    /// Variables captured from the outer scope
    captures: HashMap<IdentifierNode, Type>,
}

impl<'a, 'b> CaptureAnalyzerVisitor<'a, 'b> {
    fn new(ctx: &'a mut HIRContext<'b>) -> Self {
        Self {
            ctx,
            local_scope: HashSet::new(),
            captures: HashMap::new(),
        }
    }
}

impl<'a, 'b, 'ast> ASTVisitor<'ast> for CaptureAnalyzerVisitor<'a, 'b> {
    fn visit_ident_decl(&mut self, id: IdentifierNode) {
        self.local_scope.insert(id);
    }

    fn visit_ident_usage(&mut self, id: IdentifierNode) {
        if self.local_scope.contains(&id) {
            return;
        }

        if self.captures.contains_key(&id) {
            return;
        }

        if let Some(decl_id) = self.ctx.module_builder.scope_lookup(id.name) {
            let decl = self.ctx.program_builder.get_declaration(decl_id);
            if let CheckedDeclaration::Var(var_decl) = decl {
                self.captures.insert(id, var_decl.constraint.clone());
            }
        }
    }

    fn visit_fn_expr(&mut self, decl: &'ast FnDecl) {
        let mut nested_visitor = CaptureAnalyzerVisitor::new(self.ctx);

        for param in &decl.params {
            nested_visitor.local_scope.insert(param.identifier);
        }

        nested_visitor.visit_block(&decl.body);

        for (id, ty) in nested_visitor.captures {
            if !self.local_scope.contains(&id) {
                self.captures.entry(id).or_insert(ty);
            }
        }
    }
}

pub fn analyze_captures(
    ctx: &mut HIRContext,
    params: &[CheckedParam],
    body: &BlockContents,
) -> HashMap<IdentifierNode, Type> {
    let mut visitor = CaptureAnalyzerVisitor::new(ctx);

    for p in params {
        visitor.local_scope.insert(p.identifier);
    }

    visitor.visit_block(body);

    visitor.captures
}
