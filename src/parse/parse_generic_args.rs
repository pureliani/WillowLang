use crate::{
    ast::{base::base_type::TypeAnnotation, Span},
    tokenize::{PunctuationKind, TokenKind},
};

use super::{Parser, ParsingError};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_optional_generic_args(
        &mut self,
    ) -> Result<(Vec<TypeAnnotation>, Span), ParsingError<'a>> {
        let start_offset = self.offset;
        if self.match_token(0, TokenKind::Punctuation(PunctuationKind::Lt)) {
            self.advance();
            let result = self.comma_separated(
                |p| p.parse_type_annotation(0),
                |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::Gt)),
            )?;
            self.consume_punctuation(PunctuationKind::Gt)?;
            let span = self.get_span(start_offset, self.offset - 1)?;

            return Ok((result, span));
        }
        let span = self.get_span(start_offset, self.offset - 1)?;

        Ok((vec![], span))
    }
}
