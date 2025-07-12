use crate::{
    ast::{
        decl::EnumDecl,
        stmt::{Stmt, StmtKind},
    },
    parse::{DocAnnotation, Parser, ParsingError},
    tokenize::{KeywordKind, PunctuationKind, TokenKind},
};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_enum_decl(&mut self, documentation: Option<DocAnnotation>) -> Result<Stmt, ParsingError<'a>> {
        let start_offset = self.offset;

        self.consume_keyword(KeywordKind::Enum)?;

        let identifier = self.consume_identifier()?;
        let generic_params = self.parse_optional_generic_params()?;

        self.consume_punctuation(PunctuationKind::LBrace)?;

        let variants = self.comma_separated(
            |p| {
                let name = p.consume_identifier()?;
                let ty = if p.match_token(0, TokenKind::Punctuation(PunctuationKind::LParen)) {
                    p.consume_punctuation(PunctuationKind::LParen)?;
                    let ty = p.parse_type_annotation(0)?;
                    p.consume_punctuation(PunctuationKind::LParen)?;
                    Some(ty)
                } else {
                    None
                };

                Ok((name, ty))
            },
            |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::RBrace)),
        )?;

        self.consume_punctuation(PunctuationKind::RBrace)?;

        let span = self.get_span(start_offset, self.offset - 1)?;

        Ok(Stmt {
            kind: StmtKind::EnumDecl(EnumDecl {
                documentation,
                generic_params,
                identifier,
                variants,
            }),
            span,
        })
    }
}
