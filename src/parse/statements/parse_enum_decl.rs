use crate::{
    ast::base::{
        base_declaration::EnumDecl,
        base_statement::{Stmt, StmtKind},
    },
    parse::{DocAnnotation, Parser, ParsingError},
    tokenizer::{KeywordKind, PunctuationKind, TokenKind},
};

impl Parser {
    pub fn parse_enum_decl(
        &mut self,
        documentation: Option<DocAnnotation>,
    ) -> Result<Stmt, ParsingError> {
        let start_offset = self.offset;

        self.consume_keyword(KeywordKind::Enum)?;
        let identifier = self.consume_identifier()?;
        self.consume_punctuation(PunctuationKind::LBrace)?;
        let variants = self.comma_separated(
            |p| p.consume_identifier(),
            |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::RBrace)),
        )?;
        self.consume_punctuation(PunctuationKind::RBrace)?;

        let span = self.get_span(start_offset, self.offset - 1)?;

        Ok(Stmt {
            kind: StmtKind::EnumDecl(EnumDecl {
                identifier,
                documentation,
                variants,
            }),
            span,
        })
    }
}
