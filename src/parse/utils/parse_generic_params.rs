use crate::{
    ast::{decl::GenericParam, type_annotation::TypeAnnotation},
    parse::{Parser, ParsingError},
    tokenize::{PunctuationKind, TokenKind},
};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_generic_param_constraint(&mut self) -> Result<Option<TypeAnnotation>, ParsingError<'a>> {
        if self.match_token(0, TokenKind::Punctuation(PunctuationKind::Col)) {
            self.advance();
            Ok(Some(self.parse_type_annotation(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn parse_optional_generic_params(&mut self) -> Result<Vec<GenericParam>, ParsingError<'a>> {
        if self.match_token(0, TokenKind::Punctuation(PunctuationKind::Lt)) {
            self.advance();
            let result = self.comma_separated(
                |p| {
                    let name = p.consume_identifier()?;
                    let constraint = p.parse_generic_param_constraint()?;

                    Ok(GenericParam {
                        constraint,
                        identifier: name,
                    })
                },
                |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::Gt)),
            )?;
            self.consume_punctuation(PunctuationKind::Gt)?;

            return Ok(result);
        }

        Ok(vec![])
    }
}
