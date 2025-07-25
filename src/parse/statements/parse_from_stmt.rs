use crate::{
    ast::stmt::{Stmt, StmtKind},
    parse::{Parser, ParsingError},
    tokenize::{KeywordKind, PunctuationKind, TokenKind},
};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_from_stmt(&mut self) -> Result<Stmt, ParsingError<'a>> {
        let start_offset = self.offset;

        self.consume_keyword(KeywordKind::From)?;
        let path = self.consume_string()?;

        self.consume_punctuation(PunctuationKind::LBrace)?;
        let identifiers = self.comma_separated(
            |p| {
                let identifier = p.consume_identifier()?;
                let alias = if p.match_token(0, TokenKind::Punctuation(PunctuationKind::Col)) {
                    p.advance();
                    Some(p.consume_identifier()?)
                } else {
                    None
                };

                Ok((identifier, alias))
            },
            |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::RBrace)),
        )?;
        self.consume_punctuation(PunctuationKind::RBrace)?;

        let span = self.get_span(start_offset, self.offset - 1)?;

        return Ok(Stmt {
            kind: StmtKind::From { path, identifiers },
            span,
        });
    }
}
