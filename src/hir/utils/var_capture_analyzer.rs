use std::collections::{HashMap, HashSet};

use crate::{
    ast::{
        expr::{BlockContents, Expr, ExprKind},
        stmt::{Stmt, StmtKind},
        IdentifierNode,
    },
    hir::{
        cfg::CheckedDeclaration,
        types::{checked_declaration::CheckedParam, checked_type::Type},
        HIRContext,
    },
};

struct CaptureAnalyzer<'a, 'b> {
    ctx: &'a mut HIRContext<'b>,
    local_scope: HashSet<IdentifierNode>,
    captures: HashMap<IdentifierNode, Type>,
}

impl<'a, 'b> CaptureAnalyzer<'a, 'b> {
    fn new(ctx: &'a mut HIRContext<'b>) -> Self {
        Self {
            ctx,
            local_scope: HashSet::new(),
            captures: HashMap::new(),
        }
    }

    fn analyze_fn_body(&mut self, params: &[CheckedParam], body: &BlockContents) {
        for p in params {
            self.local_scope.insert(p.identifier);
        }
        for stmt in &body.statements {
            self.walk_stmt(stmt);
        }
        if let Some(expr) = &body.final_expr {
            self.walk_expr(expr);
        }
    }

    fn walk_stmt(&mut self, stmt: &Stmt) {
        match &stmt.kind {
            StmtKind::Expression(expr) => self.walk_expr(expr),
            StmtKind::VarDecl(decl) => {
                self.local_scope.insert(decl.identifier);
                if let Some(value) = &decl.value {
                    self.walk_expr(value);
                }
            }
            StmtKind::Return { value } => self.walk_expr(value),
            StmtKind::Assignment { target, value } => {
                self.walk_expr(target);
                self.walk_expr(value);
            }
            StmtKind::While { condition, body } => {
                self.walk_expr(condition);
                self.analyze_fn_body(&[], body);
            }
            _ => {}
        }
    }

    fn walk_expr(&mut self, expr: &Expr) {
        match &expr.kind {
            ExprKind::Identifier(identifier) => {
                if !self.local_scope.contains(identifier) {
                    if let Some(CheckedDeclaration::Var(decl)) = self
                        .ctx
                        .module_builder
                        .scope_lookup(identifier.name)
                        .cloned()
                    {
                        self.captures
                            .entry(*identifier)
                            .or_insert_with(|| decl.constraint.clone());
                    }
                }
            }
            ExprKind::Not { right } | ExprKind::Neg { right } => self.walk_expr(right),
            ExprKind::Add { left, right }
            | ExprKind::Subtract { left, right }
            | ExprKind::Multiply { left, right }
            | ExprKind::Divide { left, right }
            | ExprKind::Modulo { left, right }
            | ExprKind::LessThan { left, right }
            | ExprKind::LessThanOrEqual { left, right }
            | ExprKind::GreaterThan { left, right }
            | ExprKind::GreaterThanOrEqual { left, right }
            | ExprKind::Equal { left, right }
            | ExprKind::NotEqual { left, right }
            | ExprKind::And { left, right }
            | ExprKind::Or { left, right } => {
                self.walk_expr(left);
                self.walk_expr(right);
            }
            ExprKind::Struct(fields) => {
                for (_, value) in fields {
                    self.walk_expr(value);
                }
            }
            ExprKind::Access { left, .. } => self.walk_expr(left),
            ExprKind::TypeCast { left, .. } => self.walk_expr(left),
            ExprKind::FnCall { left, args } => {
                self.walk_expr(left);
                for arg in args {
                    self.walk_expr(arg);
                }
            }
            ExprKind::Fn(decl) => {
                let param_identifiers: Vec<IdentifierNode> =
                    decl.params.iter().map(|p| p.identifier).collect();

                let mut nested_analyzer = CaptureAnalyzer::new(self.ctx);

                for id in param_identifiers {
                    nested_analyzer.local_scope.insert(id);
                }

                nested_analyzer.analyze_fn_body(&[], &decl.body);
                for (id_node, ty) in nested_analyzer.captures {
                    self.captures.entry(id_node).or_insert(ty);
                }
            }
            ExprKind::Match { conditions, arms } => {
                for cond in conditions {
                    self.walk_expr(cond);
                }
                for arm in arms {
                    self.walk_expr(&arm.expression);
                }
            }
            ExprKind::If {
                branches,
                else_branch,
            } => {
                for (cond, body) in branches {
                    self.walk_expr(cond);
                    self.analyze_fn_body(&[], body);
                }
                if let Some(body) = else_branch {
                    self.analyze_fn_body(&[], body);
                }
            }
            ExprKind::List(items) => {
                for item in items {
                    self.walk_expr(item);
                }
            }
            ExprKind::CodeBlock(body) => self.analyze_fn_body(&[], body),
            _ => {}
        }
    }
}

pub fn analyze_captures(
    ctx: &mut HIRContext,
    params: &[CheckedParam],
    body: &BlockContents,
) -> HashMap<IdentifierNode, Type> {
    let mut analyzer = CaptureAnalyzer::new(ctx);
    analyzer.analyze_fn_body(params, body);
    analyzer.captures
}
