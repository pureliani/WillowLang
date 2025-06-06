use crate::{
    ast::base::{
        base_declaration::TypeAliasDecl,
        base_statement::{Stmt, StmtKind},
    },
    parse::{DocAnnotation, Parser, ParsingError},
    tokenize::{KeywordKind, PunctuationKind},
};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_type_alias_decl(&mut self, documentation: Option<DocAnnotation>) -> Result<Stmt, ParsingError<'a>> {
        let start_offset = self.offset;

        self.consume_keyword(KeywordKind::Type)?;

        let name = self.consume_identifier()?;
        let generic_params = self.parse_optional_generic_params()?;

        self.consume_punctuation(PunctuationKind::Eq)?;

        let ty = self.parse_type_annotation(0)?;

        self.consume_punctuation(PunctuationKind::SemiCol)?;

        let span = self.get_span(start_offset, self.offset - 1)?;

        Ok(Stmt {
            kind: StmtKind::TypeAliasDecl(TypeAliasDecl {
                identifier: name,
                documentation,
                generic_params,
                value: ty,
            }),
            span,
        })
    }
}
