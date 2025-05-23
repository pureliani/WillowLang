use crate::{
    ast::base::{
        base_declaration::Param,
        base_expression::{Expr, ExprKind},
    },
    parse::{Parser, ParsingError},
    tokenizer::{PunctuationKind, TokenKind},
};

impl Parser {
    pub fn parse_fn_expr(&mut self) -> Result<Expr, ParsingError> {
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

        let return_type = if self.match_token(0, TokenKind::Punctuation(PunctuationKind::Col)) {
            self.advance();

            let return_type = self.parse_type_annotation(0)?;
            Some(return_type)
        } else {
            None
        };

        self.consume_punctuation(PunctuationKind::FatArrow)?;

        let body = self.parse_codeblock_expr()?;

        Ok(Expr {
            kind: ExprKind::Fn {
                params,
                body,
                return_type,
                generic_params,
            },
            span: self.get_span(start_offset, self.offset - 1)?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use crate::{
        ast::{
            base::{
                base_declaration::{GenericParam, Param},
                base_expression::{BlockContents, Expr, ExprKind},
                base_type::{TypeAnnotation, TypeAnnotationKind},
            },
            IdentifierNode, Position, Span,
        },
        tokenizer::Tokenizer,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn parses_basic_function() {
        let (tokens, _) = Tokenizer::tokenize("() => {}".to_owned());
        let mut parser = Parser {
            checkpoint_offset: 0,
            offset: 0,
            tokens,
        };
        let actual_ast = parser.parse_expr(0);
        let expected_ast = Ok(Expr {
            kind: ExprKind::Fn {
                params: vec![],
                body: BlockContents {
                    final_expr: None,
                    statements: vec![],
                },
                return_type: None,
                generic_params: vec![],
            },
            span: Span {
                start: Position { line: 1, col: 1 },
                end: Position { line: 1, col: 9 },
            },
        });
        assert_eq!(actual_ast, expected_ast)
    }

    #[test]
    fn parses_function_with_params() {
        let (tokens, _) = Tokenizer::tokenize("(const a: i32) => {}".to_owned());
        let mut parser = Parser {
            checkpoint_offset: 0,
            offset: 0,
            tokens,
        };
        let actual_ast = parser.parse_expr(0);
        let expected_ast = Ok(Expr {
            kind: ExprKind::Fn {
                params: vec![Param {
                    identifier: IdentifierNode {
                        name: String::from("a"),
                        span: Span {
                            start: Position { line: 0, col: 0 },
                            end: Position { line: 0, col: 0 },
                        },
                    },
                    constraint: TypeAnnotation {
                        kind: TypeAnnotationKind::I32,
                        span: Span {
                            start: Position { line: 1, col: 5 },
                            end: Position { line: 1, col: 8 },
                        },
                    },
                }],
                body: BlockContents {
                    final_expr: None,
                    statements: vec![],
                },
                return_type: None,
                generic_params: vec![],
            },
            span: Span {
                start: Position { line: 1, col: 1 },
                end: Position { line: 1, col: 15 },
            },
        });
        assert_eq!(actual_ast, expected_ast)
    }

    #[test]
    fn parses_function_with_generic_params() {
        let (tokens, _) = Tokenizer::tokenize("<AParam>(a: AParam) => {}".to_owned());
        let mut parser = Parser {
            checkpoint_offset: 0,
            offset: 0,
            tokens,
        };
        let actual_ast = parser.parse_expr(0);
        let expected_ast = Ok(Expr {
            kind: ExprKind::Fn {
                params: vec![Param {
                    identifier: IdentifierNode {
                        name: String::from("a"),
                        span: Span {
                            start: Position { line: 0, col: 0 },
                            end: Position { line: 0, col: 0 },
                        },
                    },
                    constraint: TypeAnnotation {
                        kind: TypeAnnotationKind::Identifier(IdentifierNode {
                            name: String::from("AParam"),
                            span: Span {
                                start: Position { line: 0, col: 0 },
                                end: Position { line: 0, col: 0 },
                            },
                        }),
                        span: Span {
                            start: Position { line: 1, col: 13 },
                            end: Position { line: 1, col: 19 },
                        },
                    },
                }],
                body: BlockContents {
                    final_expr: None,
                    statements: vec![],
                },
                return_type: None,
                generic_params: vec![GenericParam {
                    constraint: None,
                    identifier: IdentifierNode {
                        name: String::from("AParam"),
                        span: Span {
                            start: Position { line: 0, col: 0 },
                            end: Position { line: 0, col: 0 },
                        },
                    },
                }],
            },
            span: Span {
                start: Position { line: 1, col: 1 },
                end: Position { line: 1, col: 26 },
            },
        });
        assert_eq!(actual_ast, expected_ast)
    }

    #[test]
    fn parses_function_with_return_type() {
        let (tokens, _) = Tokenizer::tokenize("<AParam>(a: AParam): i32 => {}".to_owned());
        let mut parser = Parser {
            checkpoint_offset: 0,
            offset: 0,
            tokens,
        };
        let actual_ast = parser.parse_expr(0);
        let expected_ast = Ok(Expr {
            kind: ExprKind::Fn {
                params: vec![Param {
                    identifier: IdentifierNode {
                        name: String::from("a"),
                        span: Span {
                            start: Position { line: 0, col: 0 },
                            end: Position { line: 0, col: 0 },
                        },
                    },
                    constraint: TypeAnnotation {
                        kind: TypeAnnotationKind::Identifier(IdentifierNode {
                            name: String::from("AParam"),
                            span: Span {
                                start: Position { line: 0, col: 0 },
                                end: Position { line: 0, col: 0 },
                            },
                        }),
                        span: Span {
                            start: Position { line: 1, col: 13 },
                            end: Position { line: 1, col: 19 },
                        },
                    },
                }],
                body: BlockContents {
                    final_expr: None,
                    statements: vec![],
                },
                return_type: Some(TypeAnnotation {
                    kind: TypeAnnotationKind::I32,
                    span: Span {
                        start: Position { line: 1, col: 22 },
                        end: Position { line: 1, col: 25 },
                    },
                }),
                generic_params: vec![GenericParam {
                    constraint: None,
                    identifier: IdentifierNode {
                        name: String::from("AParam"),
                        span: Span {
                            start: Position { line: 0, col: 0 },
                            end: Position { line: 0, col: 0 },
                        },
                    },
                }],
            },
            span: Span {
                start: Position { line: 1, col: 1 },
                end: Position { line: 1, col: 31 },
            },
        });
        assert_eq!(actual_ast, expected_ast)
    }
}
