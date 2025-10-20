use crate::{
    ast::type_annotation::TypeAnnotation, parse::ParsingError, tokenize::PunctuationKind,
};

use super::Parser;

impl Parser {
    pub fn parse_parenthesized_type_annotation(
        &mut self,
    ) -> Result<TypeAnnotation, ParsingError> {
        self.consume_punctuation(PunctuationKind::LParen)?;
        let item = self.parse_type_annotation(0)?;
        self.consume_punctuation(PunctuationKind::RParen)?;

        Ok(item)
    }
}
