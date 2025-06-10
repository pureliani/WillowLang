use crate::{
    ast::base::base_type::TypeAnnotation,
    tokenize::{PunctuationKind, TokenKind},
};

use super::{Parser, ParsingError};

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
