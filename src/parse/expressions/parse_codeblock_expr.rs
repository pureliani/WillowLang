// codeblock_parser (fixed)
use crate::{
    ast::base::{base_expression::BlockContents, base_statement::StmtKind},
    parse::{Parser, ParsingError},
    tokenize::{PunctuationKind, TokenKind},
};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_codeblock_expr(&mut self) -> Result<BlockContents, ParsingError<'a>> {
        let start_offset = self.offset;
        self.consume_punctuation(PunctuationKind::LBrace)?;

        let mut statements = Vec::new();
        let mut final_expr = None;

        loop {
            if self.match_token(0, TokenKind::Punctuation(PunctuationKind::RBrace)) {
                break;
            }

            let stmt = self.parse_stmt()?;

            if let StmtKind::Expression(expr) = &stmt.kind {
                if self.match_token(0, TokenKind::Punctuation(PunctuationKind::RBrace)) {
                    final_expr = Some(Box::new(expr.clone()));
                    break;
                }
            }

            statements.push(stmt);

            if self.match_token(0, TokenKind::Punctuation(PunctuationKind::RBrace)) {
                break;
            }
        }

        self.consume_punctuation(PunctuationKind::RBrace)?;

        let span = self.get_span(start_offset, self.offset - 1)?;

        Ok(BlockContents {
            statements,
            final_expr,
            span,
        })
    }
}
