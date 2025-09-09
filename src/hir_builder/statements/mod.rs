pub mod assignment;
pub mod enum_decl;
pub mod expression;
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
    hir_builder::{expressions::r#if::IfContext, FunctionBuilder, ModuleBuilder},
};

impl FunctionBuilder {
    pub fn build_statements(&mut self, module_builder: &mut ModuleBuilder, statements: Vec<Stmt>) {
        for statement in statements {
            match statement.kind {
                StmtKind::Expression(expr) => {
                    if let ExprKind::If { branches, else_branch } = expr.kind {
                        self.build_if(branches, else_branch, IfContext::Statement);
                    } else {
                        self.build_expr(expr);
                    }
                }
                StmtKind::TypeAliasDecl(type_alias_decl) => todo!(),
                StmtKind::VarDecl(var_decl) => todo!(),
                StmtKind::Return { value } => todo!(),
                StmtKind::Assignment { target, value } => todo!(),
                StmtKind::From { path, identifiers } => todo!(),
                StmtKind::While { condition, body } => todo!(),
                StmtKind::Break => todo!(),
                StmtKind::Continue => todo!(),
                StmtKind::StructDecl(struct_decl) => todo!(),
                StmtKind::UnionDecl(union_decl) => todo!(),
            }
        }
    }
}
