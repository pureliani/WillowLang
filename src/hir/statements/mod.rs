pub mod assignment;
pub mod enum_decl;
pub mod from;
pub mod r#return;
pub mod type_alias_decl;
pub mod var_decl;
pub mod r#while;

use crate::{
    ast::{
        expr::ExprKind,
        stmt::{Stmt, StmtKind},
    },
    hir::{expressions::r#if::IfContext, FunctionBuilder, HIRContext},
};

impl FunctionBuilder {
    pub fn build_statements(&mut self, ctx: &mut HIRContext, statements: Vec<Stmt>) {
        for statement in statements {
            match statement.kind {
                StmtKind::Expression(expr) => {
                    if let ExprKind::If {
                        branches,
                        else_branch,
                    } = expr.kind
                    {
                        self.build_if(ctx, branches, else_branch, IfContext::Statement);
                    } else {
                        self.build_expr(ctx, expr);
                    }
                }
                StmtKind::TypeAliasDecl(type_alias_decl) => {
                    self.build_type_alias_decl(ctx, type_alias_decl, statement.span);
                }
                StmtKind::VarDecl(var_decl) => todo!(),
                StmtKind::Return { value } => todo!(),
                StmtKind::Assignment { target, value } => {
                    self.build_assignment_stmt(ctx, target, value)
                }
                StmtKind::From { path, identifiers } => todo!(),
                StmtKind::While { condition, body } => todo!(),
                StmtKind::Break => todo!(),
                StmtKind::Continue => todo!(),
                StmtKind::EnumDecl(enum_decl) => todo!(),
            }
        }
    }
}
