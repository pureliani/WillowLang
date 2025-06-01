pub mod parse_fn_type_annotation;
pub mod parse_parenthesized_type_annotation;

use super::{Parser, ParsingError, ParsingErrorKind};
use crate::{
    ast::{
        base::base_type::{TypeAnnotation, TypeAnnotationKind},
        Span,
    },
    tokenize::{KeywordKind, PunctuationKind, TokenKind},
};

fn infix_bp(token_kind: &TokenKind) -> Option<(u8, u8)> {
    use PunctuationKind::*;
    use TokenKind::*;

    let priority = match token_kind {
        Punctuation(Or) => (1, 2),
        _ => return None,
    };

    Some(priority)
}

fn suffix_bp(token_kind: &TokenKind) -> Option<(u8, ())> {
    use PunctuationKind::*;
    use TokenKind::*;

    let priority = match token_kind {
        Punctuation(Lt) => (3, ()),
        _ => return None,
    };

    Some(priority)
}

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_type_annotation(
        &mut self,
        min_prec: u8,
    ) -> Result<TypeAnnotation, ParsingError<'a>> {
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
            TokenKind::Keyword(KeywordKind::Null) => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::Null)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::Null,
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
            TokenKind::Keyword(KeywordKind::USize) => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::USize)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::USize,
                    span,
                }
            }
            TokenKind::Keyword(KeywordKind::ISize) => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::ISize)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::ISize,
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
            TokenKind::Keyword(KeywordKind::Char) => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::Char)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::Char,
                    span,
                }
            }
            TokenKind::Punctuation(PunctuationKind::LParen) => {
                self.place_checkpoint();
                let type_annotation = self.parse_fn_type_annotation().or_else(|fn_err| {
                    let offset_after_fn_attempt = self.offset;
                    self.goto_checkpoint();
                    self.parse_parenthesized_type_annotation()
                        .or_else(|paren_err| {
                            let offset_after_paren_attempt = self.offset;
                            if offset_after_fn_attempt >= offset_after_paren_attempt {
                                Err(fn_err)
                            } else {
                                Err(paren_err)
                            }
                        })
                })?;

                type_annotation
            }

            TokenKind::Punctuation(PunctuationKind::LBracket) => {
                let start_offset = self.offset;

                self.consume_punctuation(PunctuationKind::LBracket)?;
                let ty = self.parse_type_annotation(0)?;
                self.consume_punctuation(PunctuationKind::SemiCol)?;
                let size = self.consume_number()?;
                self.consume_punctuation(PunctuationKind::RBracket)?;

                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::Array {
                        item_type: Box::new(ty),
                        size,
                    },
                    span,
                }
            }
            TokenKind::Identifier(_) => {
                let start_offset = self.offset;

                let id = self.consume_identifier()?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::Identifier(id),
                    span,
                }
            }
            _ => {
                return Err(ParsingError {
                    kind: ParsingErrorKind::ExpectedATypeButFound(token.clone()),
                    span: token.span,
                })
            }
        };

        loop {
            let op = match self.current() {
                Some(o) => o,
                None => break,
            };

            if let Some((left_prec, ())) = suffix_bp(&op.kind) {
                if left_prec < min_prec {
                    break;
                }

                lhs = match op.kind {
                    TokenKind::Punctuation(PunctuationKind::Lt) => {
                        let (generic_args, generic_args_span) =
                            self.parse_optional_generic_args()?;

                        TypeAnnotation {
                            kind: TypeAnnotationKind::GenericApply {
                                left: Box::new(lhs.clone()),
                                args: generic_args,
                            },
                            span: generic_args_span,
                        }
                    }
                    _ => break,
                };

                continue;
            }

            if let Some((left_prec, right_prec)) = infix_bp(&op.kind) {
                if left_prec < min_prec {
                    break;
                }

                lhs = match op.kind {
                    TokenKind::Punctuation(PunctuationKind::Or) => {
                        let start_offset = self.offset;

                        self.advance();
                        let rhs = self.parse_type_annotation(right_prec)?;
                        let end_span = self.get_span(start_offset, self.offset - 1)?;
                        let span = Span {
                            start: lhs.span.start,
                            end: end_span.end,
                        };

                        let kind =
                            if let TypeAnnotationKind::Union(existing_variants) = &mut lhs.kind {
                                existing_variants.push(rhs);
                                lhs.kind
                            } else {
                                TypeAnnotationKind::Union(vec![lhs, rhs])
                            };

                        TypeAnnotation { kind, span }
                    }
                    _ => break,
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
    use crate::{
        ast::{
            base::base_type::{TypeAnnotation, TypeAnnotationKind},
            Span,
        },
        compile::string_interner::StringInterner,
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
                "usize",
                TypeAnnotation {
                    kind: TypeAnnotationKind::USize,
                    span: Span {
                        start: Position {
                            line: 1,
                            col: 1,
                            byte_offset: 0,
                        },
                        end: Position {
                            line: 1,
                            col: 6,
                            byte_offset: 5,
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
                "null",
                TypeAnnotation {
                    kind: TypeAnnotationKind::Null,
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
                "char",
                TypeAnnotation {
                    kind: TypeAnnotationKind::Char,
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
        ];

        for (input, expected) in test_cases {
            let (tokens, _) = Tokenizer::tokenize(input);
            let mut parser = Parser {
                offset: 0,
                checkpoint_offset: 0,
                tokens,
                interner: &mut StringInterner::new(),
            };
            let result = parser.parse_type_annotation(0);

            assert_eq!(result, Ok(expected))
        }
    }

    #[test]
    fn parses_union_types() {
        use crate::ast::Position;
        use crate::tokenize::Tokenizer;
        use pretty_assertions::assert_eq;

        let (tokens, _) = Tokenizer::tokenize("i8 | i16 | i32 | i64");
        let mut parser = Parser {
            offset: 0,
            checkpoint_offset: 0,
            tokens,
            interner: &mut StringInterner::new(),
        };
        let result = parser.parse_type_annotation(0);

        assert_eq!(
            result,
            Ok(TypeAnnotation {
                kind: TypeAnnotationKind::Union(vec![
                    TypeAnnotation {
                        kind: TypeAnnotationKind::I8,
                        span: Span {
                            start: Position {
                                line: 1,
                                col: 1,
                                byte_offset: 0
                            },
                            end: Position {
                                line: 1,
                                col: 3,
                                byte_offset: 2
                            }
                        }
                    },
                    TypeAnnotation {
                        kind: TypeAnnotationKind::I16,
                        span: Span {
                            start: Position {
                                line: 1,
                                col: 6,
                                byte_offset: 5
                            },
                            end: Position {
                                line: 1,
                                col: 9,
                                byte_offset: 8
                            }
                        }
                    },
                    TypeAnnotation {
                        kind: TypeAnnotationKind::I32,
                        span: Span {
                            start: Position {
                                line: 1,
                                col: 12,
                                byte_offset: 11
                            },
                            end: Position {
                                line: 1,
                                col: 15,
                                byte_offset: 14
                            }
                        }
                    },
                    TypeAnnotation {
                        kind: TypeAnnotationKind::I64,
                        span: Span {
                            start: Position {
                                line: 1,
                                col: 18,
                                byte_offset: 17
                            },
                            end: Position {
                                line: 1,
                                col: 21,
                                byte_offset: 20
                            }
                        }
                    }
                ]),
                span: Span {
                    start: Position {
                        line: 1,
                        col: 1,
                        byte_offset: 0
                    },
                    end: Position {
                        line: 1,
                        col: 21,
                        byte_offset: 20
                    }
                }
            })
        )
    }
}
