use crate::{
    ast::{
        decl::{UnionDecl, UnionDeclVariant},
        stmt::{Stmt, StmtKind},
    },
    parse::{DocAnnotation, Parser, ParsingError},
    tokenize::{KeywordKind, PunctuationKind, TokenKind},
};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_union_decl(&mut self, documentation: Option<DocAnnotation>) -> Result<Stmt, ParsingError<'a>> {
        let start_offset = self.offset;

        self.consume_keyword(KeywordKind::Union)?;
        let identifier = self.consume_identifier()?;
        self.consume_punctuation(PunctuationKind::LBrace)?;
        let variants = self.comma_separated(
            |p| {
                let name = p.consume_identifier()?;
                let payload = if p.match_token(0, TokenKind::Punctuation(PunctuationKind::LParen)) {
                    p.consume_punctuation(PunctuationKind::LParen)?;
                    let ty = p.parse_type_annotation(0)?;
                    p.consume_punctuation(PunctuationKind::RParen)?;
                    Some(ty)
                } else {
                    None
                };

                Ok(UnionDeclVariant { name, payload })
            },
            |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::RBrace)),
        )?;
        self.consume_punctuation(PunctuationKind::RBrace)?;

        let span = self.get_span(start_offset, self.offset - 1)?;

        Ok(Stmt {
            kind: StmtKind::UnionDecl(UnionDecl {
                documentation,
                identifier,
                variants,
            }),
            span,
        })
    }
}
