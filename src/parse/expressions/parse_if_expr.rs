use crate::{
    ast::expr::{BlockContents, Expr, ExprKind},
    parse::{Parser, ParsingError},
    tokenize::{KeywordKind, TokenKind},
};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_if_expr(&mut self) -> Result<Expr, ParsingError<'a>> {
        let start_offset = self.offset;
        let mut branches: Vec<(Box<Expr>, BlockContents)> = Vec::new();

        self.consume_keyword(KeywordKind::If)?;
        let condition = self.parse_expr(0)?;
        let then_branch = self.parse_codeblock_expr()?;
        branches.push((Box::new(condition), then_branch));

        while self.match_token(0, TokenKind::Keyword(KeywordKind::Else))
            && self.match_token(1, TokenKind::Keyword(KeywordKind::If))
        {
            self.advance();
            self.advance();

            let else_if_condition = self.parse_expr(0)?;
            let else_if_body = self.parse_codeblock_expr()?;
            branches.push((Box::new(else_if_condition), else_if_body));
        }

        let else_branch = if self.match_token(0, TokenKind::Keyword(KeywordKind::Else)) {
            self.advance();

            let else_body = self.parse_codeblock_expr()?;
            Some(else_body)
        } else {
            None
        };

        Ok(Expr {
            kind: ExprKind::If { branches, else_branch },
            span: self.get_span(start_offset, self.offset - 1)?,
        })
    }
}
