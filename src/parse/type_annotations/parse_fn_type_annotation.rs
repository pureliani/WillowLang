use crate::{
    ast::base::{
        base_declaration::Param,
        base_type::{TypeAnnotation, TypeAnnotationKind},
    },
    parse::ParsingError,
    tokenizer::{PunctuationKind, TokenKind},
};

use super::Parser;

impl Parser {
    pub fn parse_fn_type_annotation(&mut self) -> Result<TypeAnnotation, ParsingError> {
        let start_offset = self.offset;

        let generic_params = self.parse_optional_generic_params()?;
        self.consume_punctuation(PunctuationKind::LParen)?;
        let params = self.comma_separated(
            |p| {
                let identifier = p.consume_identifier()?;
                p.consume_punctuation(PunctuationKind::Col)?;
                let constraint = p.parse_type_annotation(0)?;

                Ok(Param {
                    constraint,
                    identifier,
                })
            },
            |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::RParen)),
        )?;
        self.consume_punctuation(PunctuationKind::RParen)?;

        self.consume_punctuation(PunctuationKind::FatArrow)?;

        let return_type = Box::new(self.parse_type_annotation(0)?);

        let span = self.get_span(start_offset, self.offset - 1)?;

        let type_kind = if generic_params.is_empty() {
            TypeAnnotationKind::FnType {
                params,
                return_type,
            }
        } else {
            TypeAnnotationKind::GenericFnType {
                params,
                return_type,
                generic_params,
            }
        };

        Ok(TypeAnnotation {
            kind: type_kind,
            span,
        })
    }
}
