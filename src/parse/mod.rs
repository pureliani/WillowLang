mod expressions;
mod parse_generic_args;
mod parse_generic_params;
mod statements;
mod type_annotations;

pub struct Parser<'a, 'b: 'a> {
    pub offset: usize,
    pub tokens: Vec<Token<'a>>,
    pub checkpoint_offset: usize,
    pub interner: &'b mut StringInterner<'b>,
}

use unicode_segmentation::UnicodeSegmentation;

use crate::{
    ast::{base::base_statement::Stmt, IdentifierNode, Position, Span, StringNode},
    compile::string_interner::{InternerId, StringInterner},
    tokenize::{KeywordKind, NumberKind, PunctuationKind, Token, TokenKind},
};

#[derive(Debug, Clone, PartialEq)]
pub enum ParsingErrorKind<'a> {
    DocMustBeFollowedByDeclaration,
    ExpectedAnExpressionButFound(&'a Token<'a>),
    ExpectedATypeButFound(&'a Token<'a>),
    InvalidSuffixOperator(&'a Token<'a>),
    UnexpectedEndOfInput,
    ExpectedAnIdentifier,
    ExpectedAPunctuationMark(PunctuationKind),
    ExpectedAKeyword(KeywordKind),
    ExpectedAStringValue,
    ExpectedANumericValue,
    UnknownStaticMethod(IdentifierNode),
    UnexpectedStatementAfterFinalExpression,
    ExpectedStatementOrExpression { found: &'a TokenKind<'a> },
    UnexpectedTokenAfterFinalExpression { found: &'a TokenKind<'a> },
}

impl<'a> ParsingErrorKind<'a> {
    pub fn code(&self) -> usize {
        match self {
            ParsingErrorKind::DocMustBeFollowedByDeclaration { .. } => 1,
            ParsingErrorKind::ExpectedAnExpressionButFound(..) => 2,
            ParsingErrorKind::ExpectedATypeButFound(..) => 3,
            ParsingErrorKind::InvalidSuffixOperator(..) => 4,
            ParsingErrorKind::UnexpectedEndOfInput => 15,
            ParsingErrorKind::ExpectedAnIdentifier => 16,
            ParsingErrorKind::ExpectedAPunctuationMark(..) => 17,
            ParsingErrorKind::ExpectedAKeyword(..) => 18,
            ParsingErrorKind::ExpectedAStringValue => 19,
            ParsingErrorKind::ExpectedANumericValue => 20,
            ParsingErrorKind::UnknownStaticMethod(..) => 21,
            ParsingErrorKind::UnexpectedStatementAfterFinalExpression => 22,
            ParsingErrorKind::ExpectedStatementOrExpression { .. } => 23,
            ParsingErrorKind::UnexpectedTokenAfterFinalExpression { .. } => 24,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsingError<'a> {
    pub kind: ParsingErrorKind<'a>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DocAnnotation {
    message: InternerId,
    span: Span,
}

impl<'a, 'b> Parser<'a, 'b> {
    fn match_token(&self, index: usize, kind: TokenKind) -> bool {
        if let Some(token) = self.tokens.get(self.offset + index) {
            return token.kind == kind;
        }

        false
    }

    fn advance(&mut self) {
        self.offset += 1;
    }

    fn current(&self) -> Option<&Token<'a>> {
        self.tokens.get(self.offset)
    }

    fn unexpected_end_of_input(&self) -> ParsingError<'a> {
        // TODO: fix this
        let first_token_span = Span {
            start: Position {
                line: 1,
                col: 1,
                byte_offset: 0,
            },
            end: Position {
                line: 1,
                col: 1,
                byte_offset: 0,
            },
        };

        let last_token_span = self
            .tokens
            .last()
            .map(|t| &t.span)
            .unwrap_or(&first_token_span);

        ParsingError {
            kind: ParsingErrorKind::UnexpectedEndOfInput,
            span: *last_token_span,
        }
    }

    fn get_span(
        &mut self,
        start_offset: usize,
        end_offset: usize,
    ) -> Result<Span, ParsingError<'a>> {
        let start = self
            .tokens
            .get(start_offset)
            .ok_or(self.unexpected_end_of_input())?;

        let end = self
            .tokens
            .get(end_offset)
            .ok_or(self.unexpected_end_of_input())?;

        Ok(Span {
            start: start.span.start,
            end: end.span.end,
        })
    }

    fn place_checkpoint(&mut self) {
        self.checkpoint_offset = self.offset;
    }

    fn goto_checkpoint(&mut self) {
        self.offset = self.checkpoint_offset;
    }

    pub fn consume_string(&mut self) -> Result<StringNode, ParsingError<'a>> {
        if let Some(t) = self.current() {
            match t.clone() {
                Token {
                    kind: TokenKind::String(value),
                    span,
                } => {
                    self.advance();

                    Ok(StringNode {
                        span,
                        len: value.graphemes(true).count(),
                        value: self.interner.intern(&value),
                    })
                }
                t => {
                    return Err(ParsingError {
                        kind: ParsingErrorKind::ExpectedAStringValue,
                        span: t.span,
                    })
                }
            }
        } else {
            Err(self.unexpected_end_of_input())
        }
    }

    pub fn consume_punctuation(
        &mut self,
        expected: PunctuationKind,
    ) -> Result<(), ParsingError<'a>> {
        if let Some(token) = self.current() {
            match &token.kind {
                TokenKind::Punctuation(pk) if *pk == expected => {
                    self.advance();
                    Ok(())
                }
                _ => Err(ParsingError {
                    kind: ParsingErrorKind::ExpectedAPunctuationMark(expected),
                    span: token.span,
                }),
            }
        } else {
            Err(self.unexpected_end_of_input())
        }
    }

    pub fn consume_number(&mut self) -> Result<NumberKind, ParsingError<'a>> {
        if let Some(token) = self.current() {
            match token.kind {
                TokenKind::Number(number_kind) => {
                    self.advance();
                    return Ok(number_kind);
                }
                _ => {
                    return Err(ParsingError {
                        kind: ParsingErrorKind::ExpectedANumericValue,
                        span: token.span,
                    })
                }
            }
        }

        Err(self.unexpected_end_of_input())
    }

    pub fn consume_keyword(&mut self, expected: KeywordKind) -> Result<(), ParsingError<'a>> {
        if let Some(token) = self.current() {
            match token.kind {
                TokenKind::Keyword(keyword_kind) if keyword_kind == expected => {
                    self.advance();
                    Ok(())
                }
                _ => Err(ParsingError {
                    kind: ParsingErrorKind::ExpectedAKeyword(expected),
                    span: token.span,
                }),
            }
        } else {
            Err(self.unexpected_end_of_input())
        }
    }

    pub fn consume_identifier(&mut self) -> Result<IdentifierNode, ParsingError<'a>> {
        if let Some(token) = self.current() {
            match token.kind {
                TokenKind::Identifier(id) => {
                    let span = token.span;
                    let name = self.interner.intern(id);
                    self.advance();
                    Ok(IdentifierNode { name, span })
                }
                _ => Err(ParsingError {
                    kind: ParsingErrorKind::ExpectedAnIdentifier,
                    span: token.span,
                }),
            }
        } else {
            Err(self.unexpected_end_of_input())
        }
    }

    pub fn consume_optional_doc(&mut self) -> Option<DocAnnotation> {
        let result = if let Some(Token {
            kind: TokenKind::Doc(doc),
            span,
        }) = self.current()
        {
            Some(DocAnnotation {
                message: self.interner.intern(&doc),
                span: *span,
            })
        } else {
            None
        };

        if result.is_some() {
            self.advance();
        };

        result
    }

    pub fn comma_separated<F, T, E>(
        &mut self,
        mut parser: F,
        is_end: E,
    ) -> Result<Vec<T>, ParsingError<'a>>
    where
        F: FnMut(&mut Self) -> Result<T, ParsingError<'a>>,
        E: Fn(&Self) -> bool,
    {
        let mut items = Vec::new();

        if is_end(self) {
            return Ok(items);
        }

        let first_item = parser(self)?;
        items.push(first_item);

        loop {
            if is_end(self) {
                break;
            }

            self.consume_punctuation(PunctuationKind::Comma)?;

            if is_end(self) {
                break;
            }

            let item = parser(self)?;
            items.push(item);
        }

        Ok(items)
    }

    pub fn parse(
        tokens: Vec<Token<'a>>,
        interner: &'b mut StringInterner<'b>,
    ) -> (Vec<Stmt>, Vec<ParsingError<'a>>) {
        let mut state = Parser {
            offset: 0,
            checkpoint_offset: 0,
            tokens,
            interner,
        };

        let mut statements: Vec<Stmt> = vec![];
        let mut errors: Vec<ParsingError<'a>> = vec![];

        while state.current().is_some() {
            let stmt = state.parse_stmt();
            match stmt {
                Ok(s) => {
                    statements.push(s);
                }
                Err(e) => {
                    errors.push(e);
                }
            }
        }

        (statements, errors)
    }
}
