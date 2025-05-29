use crate::{
    ast::{base::base_declaration::GenericParam, base::base_type::TypeAnnotation},
    tokenize::{PunctuationKind, TokenKind},
};

use super::{Parser, ParsingError};

impl<'a> Parser<'a> {
    pub fn parse_generic_param_constraint(
        &mut self,
    ) -> Result<Option<TypeAnnotation>, ParsingError> {
        if self.match_token(0, TokenKind::Punctuation(PunctuationKind::Col)) {
            self.advance();
            Ok(Some(self.parse_type_annotation(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn parse_optional_generic_params(&mut self) -> Result<Vec<GenericParam>, ParsingError> {
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
            );
            self.consume_punctuation(PunctuationKind::Gt)?;

            return result;
        }

        Ok(vec![])
    }
}
