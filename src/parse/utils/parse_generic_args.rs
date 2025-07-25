use crate::{
    ast::type_annotation::TypeAnnotation,
    parse::{Parser, ParsingError},
    tokenize::{PunctuationKind, TokenKind},
};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_optional_generic_args(&mut self) -> Result<Vec<TypeAnnotation>, ParsingError<'a>> {
        if self.match_token(0, TokenKind::Punctuation(PunctuationKind::Lt)) {
            self.advance();
            let result = self.comma_separated(
                |p| p.parse_type_annotation(0),
                |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::Gt)),
            )?;
            self.consume_punctuation(PunctuationKind::Gt)?;

            return Ok(result);
        }

        Ok(vec![])
    }
}
