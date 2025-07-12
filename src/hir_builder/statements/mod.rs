pub mod assignment;
pub mod enum_decl;
pub mod expression;
pub mod from;
pub mod r#return;
pub mod type_alias_decl;
pub mod var_decl;
pub mod r#while;

use crate::{
    ast::stmt::{Stmt, StmtKind},
    hir_builder::HIRBuilder,
};

impl<'a> HIRBuilder<'a> {
    pub fn build_statements(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            match statement.kind {
                StmtKind::Expression(expr) => todo!(),
                StmtKind::TypeAliasDecl(type_alias_decl) => todo!(),
                StmtKind::EnumDecl(enum_decl) => todo!(),
                StmtKind::VarDecl(var_decl) => todo!(),
                StmtKind::Return(expr) => todo!(),
                StmtKind::Assignment { target, value } => todo!(),
                StmtKind::From { path, identifiers } => todo!(),
                StmtKind::While { condition, body } => todo!(), // TODO: implement in while.rs
                StmtKind::Break => todo!(),                     // TODO: implement in while.rs
                StmtKind::Continue => todo!(),                  // TODO: implement in while.rs
            }
        }
    }
}
