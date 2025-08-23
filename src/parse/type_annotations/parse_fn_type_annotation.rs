use crate::{
    ast::{
        decl::Param,
        type_annotation::{TypeAnnotation, TypeAnnotationKind},
    },
    parse::ParsingError,
    tokenize::{PunctuationKind, TokenKind},
};

use super::Parser;

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_fn_type_annotation(&mut self) -> Result<TypeAnnotation, ParsingError<'a>> {
        let start_offset = self.offset;

        // TODO: fix this

        self.consume_punctuation(PunctuationKind::LParen)?;
        let params = self.comma_separated(
            |p| {
                let identifier = p.consume_identifier()?;
                p.consume_punctuation(PunctuationKind::Col)?;
                let constraint = p.parse_type_annotation(0)?;

                Ok(Param { constraint, identifier })
            },
            |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::RParen)),
        )?;
        self.consume_punctuation(PunctuationKind::RParen)?;

        let return_type = Box::new(self.parse_type_annotation(0)?);

        let span = self.get_span(start_offset, self.offset - 1)?;

        Ok(TypeAnnotation {
            kind: TypeAnnotationKind::FnType { params, return_type },
            span,
        })
    }
}
