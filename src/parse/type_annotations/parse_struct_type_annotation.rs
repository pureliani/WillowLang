use crate::{
    ast::{
        decl::Param,
        type_annotation::{TypeAnnotation, TypeAnnotationKind},
    },
    parse::{Parser, ParsingError},
    tokenize::{PunctuationKind, TokenKind},
};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_struct_type(&mut self) -> Result<TypeAnnotation, ParsingError<'a>> {
        let start_offset = self.offset;
        self.consume_punctuation(PunctuationKind::LBrace)?;
        let fields = self.comma_separated(
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

        Ok(TypeAnnotation {
            kind: TypeAnnotationKind::Struct { fields },
            span,
        })
    }
}
