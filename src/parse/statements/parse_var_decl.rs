use crate::{
    ast::{
        decl::VarDecl,
        stmt::{Stmt, StmtKind},
    },
    parse::{DocAnnotation, Parser, ParsingError},
    tokenize::{KeywordKind, PunctuationKind, TokenKind},
};

impl<'a> Parser<'a> {
    pub fn parse_var_decl(
        &mut self,
        documentation: Option<DocAnnotation>,
    ) -> Result<Stmt, ParsingError> {
        let start_offset = self.offset;

        self.consume_keyword(KeywordKind::Let)?;

        let name = self.consume_identifier()?;

        let constraint =
            if self.match_token(0, TokenKind::Punctuation(PunctuationKind::Col)) {
                self.advance();
                Some(self.parse_type_annotation(0)?)
            } else {
                None
            };

        let value = if self.match_token(0, TokenKind::Punctuation(PunctuationKind::Eq)) {
            self.consume_punctuation(PunctuationKind::Eq)?;
            Some(self.parse_expr(0)?)
        } else {
            None
        };

        self.consume_punctuation(PunctuationKind::SemiCol)?;

        let span = self.get_span(start_offset, self.offset - 1)?;

        Ok(Stmt {
            kind: StmtKind::VarDecl(VarDecl {
                documentation,
                identifier: name,
                constraint,
                value,
            }),
            span,
        })
    }
}
