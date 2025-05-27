use crate::{
    ast::base::{
        base_declaration::{Param, StructDecl},
        base_statement::{Stmt, StmtKind},
    },
    parse::{DocAnnotation, Parser, ParsingError},
    tokenize::{KeywordKind, PunctuationKind, TokenKind},
};

impl Parser {
    pub fn parse_struct_decl(
        &mut self,
        documentation: Option<DocAnnotation>,
    ) -> Result<Stmt, ParsingError> {
        let start_offset = self.offset;

        self.consume_keyword(KeywordKind::Struct)?;
        let name = self.consume_identifier()?;
        let generic_params = self.parse_optional_generic_params()?;
        self.consume_punctuation(PunctuationKind::LBrace)?;
        let properties = self.comma_separated(
            |p| {
                let name = p.consume_identifier()?;
                p.consume_punctuation(PunctuationKind::Col)?;
                let constraint = p.parse_type_annotation(0)?;

                Ok(Param {
                    constraint,
                    identifier: name,
                })
            },
            |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::RBrace)),
        )?;
        self.consume_punctuation(PunctuationKind::RBrace)?;

        let span = self.get_span(start_offset, self.offset - 1)?;

        Ok(Stmt {
            kind: StmtKind::StructDecl(StructDecl {
                identifier: name,
                documentation,
                generic_params,
                properties,
            }),
            span,
        })
    }
}
