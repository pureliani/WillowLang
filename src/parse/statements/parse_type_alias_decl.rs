use crate::{
    ast::{
        decl::TypeAliasDecl,
        stmt::{Stmt, StmtKind},
    },
    parse::{DocAnnotation, Parser, ParsingError},
    tokenize::{KeywordKind, PunctuationKind},
};

impl Parser {
    pub fn parse_type_alias_decl(
        &mut self,
        documentation: Option<DocAnnotation>,
        is_exported: bool,
    ) -> Result<Stmt, ParsingError> {
        let start_offset = self.offset;

        self.consume_keyword(KeywordKind::Type)?;

        let name = self.consume_identifier()?;

        self.consume_punctuation(PunctuationKind::Eq)?;

        let ty = self.parse_type_annotation(0)?;

        self.consume_punctuation(PunctuationKind::SemiCol)?;

        let span = self.get_span(start_offset, self.offset - 1)?;

        Ok(Stmt {
            kind: StmtKind::TypeAliasDecl(TypeAliasDecl {
                identifier: name,
                documentation,
                value: ty,
                is_exported,
            }),
            span,
        })
    }
}
