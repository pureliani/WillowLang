pub mod parse_fn_type_annotation;
pub mod parse_parenthesized_type_annotation;
pub mod parse_ptr_type_annotation;
pub mod parse_struct_type_annotation;
pub mod parse_tag_type_annotation;

use super::{Parser, ParsingError, ParsingErrorKind};
use crate::{
    ast::type_annotation::{TypeAnnotation, TypeAnnotationKind},
    tokenize::{KeywordKind, PunctuationKind, TokenKind},
};

fn suffix_bp(token_kind: &TokenKind) -> Option<(u8, ())> {
    use PunctuationKind::*;
    use TokenKind::*;

    let priority = match token_kind {
        Punctuation(LBracket) => (3, ()),
        _ => return None,
    };

    Some(priority)
}

impl Parser {
    pub fn parse_type_annotation(
        &mut self,
        min_prec: u8,
    ) -> Result<TypeAnnotation, ParsingError> {
        let token = self.current().ok_or(self.unexpected_end_of_input())?;

        let mut lhs = match token.kind {
            TokenKind::Keyword(KeywordKind::Void) => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::Void)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::Void,
                    span,
                }
            }
            TokenKind::Keyword(KeywordKind::Bool) => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::Bool)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::Bool,
                    span,
                }
            }
            TokenKind::Keyword(KeywordKind::U8) => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::U8)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::U8,
                    span,
                }
            }
            TokenKind::Keyword(KeywordKind::U16) => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::U16)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::U16,
                    span,
                }
            }
            TokenKind::Keyword(KeywordKind::U32) => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::U32)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::U32,
                    span,
                }
            }
            TokenKind::Keyword(KeywordKind::U64) => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::U64)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::U64,
                    span,
                }
            }
            TokenKind::Keyword(KeywordKind::I8) => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::I8)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::I8,
                    span,
                }
            }
            TokenKind::Keyword(KeywordKind::I16) => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::I16)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::I16,
                    span,
                }
            }
            TokenKind::Keyword(KeywordKind::I32) => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::I32)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::I32,
                    span,
                }
            }
            TokenKind::Keyword(KeywordKind::I64) => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::I64)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::I64,
                    span,
                }
            }
            TokenKind::Keyword(KeywordKind::F32) => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::F32)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::F32,
                    span,
                }
            }
            TokenKind::Keyword(KeywordKind::F64) => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::F64)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::F64,
                    span,
                }
            }
            TokenKind::Punctuation(PunctuationKind::Hash) => {
                self.parse_tag_type_annotation()?
            }
            TokenKind::Punctuation(PunctuationKind::LParen) => {
                self.parse_parenthesized_type_annotation()?
            }
            TokenKind::Punctuation(PunctuationKind::LBrace) => {
                self.parse_struct_type_annotation()?
            }
            TokenKind::Keyword(KeywordKind::Fn) => self.parse_fn_type_annotation()?,
            TokenKind::Keyword(KeywordKind::Ref) => {
                self.parse_ptr_type_annotation(false)?
            }

            TokenKind::Keyword(KeywordKind::Mut) => {
                self.parse_ptr_type_annotation(true)?
            }
            TokenKind::Identifier(_) => {
                let identifier = self.consume_identifier()?;
                TypeAnnotation {
                    span: identifier.span,
                    kind: TypeAnnotationKind::Identifier(identifier),
                }
            }
            _ => {
                return Err(ParsingError {
                    kind: ParsingErrorKind::ExpectedATypeButFound(token.clone()),
                    span: token.span,
                })
            }
        };

        while let Some(op) = self.current() {
            if let Some((left_prec, ())) = suffix_bp(&op.kind) {
                if left_prec < min_prec {
                    break;
                }

                lhs = match op.kind {
                    TokenKind::Punctuation(PunctuationKind::LBracket) => {
                        self.consume_punctuation(PunctuationKind::LBracket)?;
                        self.consume_punctuation(PunctuationKind::RBracket)?;

                        let span =
                            self.get_span(lhs.span.start.byte_offset, self.offset - 1)?;
                        TypeAnnotation {
                            kind: TypeAnnotationKind::List(Box::new(lhs.clone())),
                            span,
                        }
                    }
                    _ => {
                        panic!("INTERNAL COMPILER ERROR: Unexpected suffix type-annotation operator")
                    }
                };

                continue;
            }

            break;
        }

        Ok(lhs)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        ast::{
            type_annotation::{TypeAnnotation, TypeAnnotationKind},
            Span,
        },
        compile::interner::SharedStringInterner,
        parse::Parser,
    };

    #[test]
    fn parses_primitive_types() {
        use crate::ast::Position;
        use crate::tokenize::Tokenizer;
        use pretty_assertions::assert_eq;

        let test_cases = vec![
            (
                "i8",
                TypeAnnotation {
                    kind: TypeAnnotationKind::I8,
                    span: Span {
                        start: Position {
                            line: 1,
                            col: 1,
                            byte_offset: 0,
                        },
                        end: Position {
                            line: 1,
                            col: 3,
                            byte_offset: 2,
                        },
                    },
                },
            ),
            (
                "i16",
                TypeAnnotation {
                    kind: TypeAnnotationKind::I16,
                    span: Span {
                        start: Position {
                            line: 1,
                            col: 1,
                            byte_offset: 0,
                        },
                        end: Position {
                            line: 1,
                            col: 4,
                            byte_offset: 3,
                        },
                    },
                },
            ),
            (
                "i32",
                TypeAnnotation {
                    kind: TypeAnnotationKind::I32,
                    span: Span {
                        start: Position {
                            line: 1,
                            col: 1,
                            byte_offset: 0,
                        },
                        end: Position {
                            line: 1,
                            col: 4,
                            byte_offset: 3,
                        },
                    },
                },
            ),
            (
                "i64",
                TypeAnnotation {
                    kind: TypeAnnotationKind::I64,
                    span: Span {
                        start: Position {
                            line: 1,
                            col: 1,
                            byte_offset: 0,
                        },
                        end: Position {
                            line: 1,
                            col: 4,
                            byte_offset: 3,
                        },
                    },
                },
            ),
            (
                "f32",
                TypeAnnotation {
                    kind: TypeAnnotationKind::F32,
                    span: Span {
                        start: Position {
                            line: 1,
                            col: 1,
                            byte_offset: 0,
                        },
                        end: Position {
                            line: 1,
                            col: 4,
                            byte_offset: 3,
                        },
                    },
                },
            ),
            (
                "f64",
                TypeAnnotation {
                    kind: TypeAnnotationKind::F64,
                    span: Span {
                        start: Position {
                            line: 1,
                            col: 1,
                            byte_offset: 0,
                        },
                        end: Position {
                            line: 1,
                            col: 4,
                            byte_offset: 3,
                        },
                    },
                },
            ),
            (
                "u8",
                TypeAnnotation {
                    kind: TypeAnnotationKind::U8,
                    span: Span {
                        start: Position {
                            line: 1,
                            col: 1,
                            byte_offset: 0,
                        },
                        end: Position {
                            line: 1,
                            col: 3,
                            byte_offset: 2,
                        },
                    },
                },
            ),
            (
                "u16",
                TypeAnnotation {
                    kind: TypeAnnotationKind::U16,
                    span: Span {
                        start: Position {
                            line: 1,
                            col: 1,
                            byte_offset: 0,
                        },
                        end: Position {
                            line: 1,
                            col: 4,
                            byte_offset: 3,
                        },
                    },
                },
            ),
            (
                "u32",
                TypeAnnotation {
                    kind: TypeAnnotationKind::U32,
                    span: Span {
                        start: Position {
                            line: 1,
                            col: 1,
                            byte_offset: 0,
                        },
                        end: Position {
                            line: 1,
                            col: 4,
                            byte_offset: 3,
                        },
                    },
                },
            ),
            (
                "u64",
                TypeAnnotation {
                    kind: TypeAnnotationKind::U64,
                    span: Span {
                        start: Position {
                            line: 1,
                            col: 1,
                            byte_offset: 0,
                        },
                        end: Position {
                            line: 1,
                            col: 4,
                            byte_offset: 3,
                        },
                    },
                },
            ),
            (
                "void",
                TypeAnnotation {
                    kind: TypeAnnotationKind::Void,
                    span: Span {
                        start: Position {
                            line: 1,
                            col: 1,
                            byte_offset: 0,
                        },
                        end: Position {
                            line: 1,
                            col: 5,
                            byte_offset: 4,
                        },
                    },
                },
            ),
            (
                "bool",
                TypeAnnotation {
                    kind: TypeAnnotationKind::Bool,
                    span: Span {
                        start: Position {
                            line: 1,
                            col: 1,
                            byte_offset: 0,
                        },
                        end: Position {
                            line: 1,
                            col: 5,
                            byte_offset: 4,
                        },
                    },
                },
            ),
            (
                "string",
                TypeAnnotation {
                    kind: TypeAnnotationKind::String,
                    span: Span {
                        start: Position {
                            line: 1,
                            col: 1,
                            byte_offset: 0,
                        },
                        end: Position {
                            line: 1,
                            col: 7,
                            byte_offset: 6,
                        },
                    },
                },
            ),
        ];

        for (input, expected) in test_cases {
            let interner = Arc::new(SharedStringInterner::default());
            let (tokens, _) = Tokenizer::tokenize(input, interner.clone());
            let mut parser = Parser {
                offset: 0,
                checkpoint_offset: 0,
                tokens,
                interner,
            };
            let result = parser.parse_type_annotation(0);

            assert_eq!(result, Ok(expected))
        }
    }
}
