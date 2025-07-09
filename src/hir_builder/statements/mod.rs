use crate::{
    ast::base::base_statement::{Stmt, StmtKind},
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
                StmtKind::Break => todo!(),
                StmtKind::Continue => todo!(),
                StmtKind::Return(expr) => todo!(),
                StmtKind::Assignment { target, value } => todo!(),
                StmtKind::From { path, identifiers } => todo!(),
                StmtKind::While { condition, body } => todo!(),
            }
        }
    }
}
