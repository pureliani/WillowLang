use crate::{
    ast::{
        expr::Expr,
        stmt::{Stmt, StmtKind},
    },
    parse::{Parser, ParsingError},
};

impl<'a> Parser<'a> {
    pub fn parse_expr_stmt(&mut self, lhs: Expr) -> Result<Stmt, ParsingError> {
        Ok(Stmt {
            span: lhs.span,
            kind: StmtKind::Expression(lhs),
        })
    }
}
