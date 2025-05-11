mod expressions;
mod parse_generic_args;
mod parse_generic_params;
mod statements;
mod type_annotations;

pub struct Parser {
    pub offset: usize,
    pub tokens: Vec<Token>,
    pub checkpoint_offset: usize,
}

use crate::{
    ast::{
        base::base_statement::{Stmt, StmtKind},
        IdentifierNode, Position, Span, StringNode,
    },
    tokenizer::{KeywordKind, NumberKind, PunctuationKind, Token, TokenKind},
};

#[derive(Debug, Clone, PartialEq)]
pub enum ParsingErrorKind {
    DocMustBeFollowedByDeclaration,
    ExpectedNumberOfArguments(usize),
    ExpectedAnExpressionButFound(Token),
    ExpectedATypeButFound(Token),
    InvalidTypeOperator(Token),
    InvalidPrefixOperator(Token),
    InvalidSuffixOperator(Token),
    InvalidInfixOperator(Token),
    InvalidArraySize,
    InvalidArrayIndex,
    UnexpectedToken(Token),
    InvalidImportPath,
    InvalidDocumentationString,
    MissingElseBranch,
    UnexpectedEndOfInput,
    ExpectedAnIdentifier,
    ExpectedAPunctuationMark(PunctuationKind),
    ExpectedAKeyword(KeywordKind),
    ExpectedAStringValue,
    ExpectedANumericValue,
    UnknownStaticMethod(IdentifierNode),
    UnexpectedStatementAfterFinalExpression,
    ExpectedStatementOrExpression { found: TokenKind },
    UnexpectedTokenAfterFinalExpression { found: TokenKind },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsingError {
    pub kind: ParsingErrorKind,
    pub span: Span,
    code: usize,
}

impl ParsingError {
    fn kind_to_code(kind: &ParsingErrorKind) -> usize {
        match kind {
            ParsingErrorKind::DocMustBeFollowedByDeclaration { .. } => 1,
            ParsingErrorKind::ExpectedNumberOfArguments(..) => 2,
            ParsingErrorKind::ExpectedAnExpressionButFound(..) => 3,
            ParsingErrorKind::ExpectedATypeButFound(..) => 4,
            ParsingErrorKind::InvalidTypeOperator(..) => 5,
            ParsingErrorKind::InvalidPrefixOperator(..) => 6,
            ParsingErrorKind::InvalidSuffixOperator(..) => 7,
            ParsingErrorKind::InvalidInfixOperator(..) => 8,
            ParsingErrorKind::InvalidArraySize => 9,
            ParsingErrorKind::InvalidArrayIndex => 10,
            ParsingErrorKind::UnexpectedToken(..) => 11,
            ParsingErrorKind::InvalidImportPath => 12,
            ParsingErrorKind::InvalidDocumentationString => 13,
            ParsingErrorKind::MissingElseBranch => 14,
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

    fn new(kind: ParsingErrorKind, span: Span) -> ParsingError {
        ParsingError {
            code: Self::kind_to_code(&kind),
            kind,
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DocAnnotation {
    message: String,
    span: Span,
}

impl Parser {
    fn match_token(&self, index: usize, kind: TokenKind) -> bool {
        if let Some(token) = self.tokens.get(self.offset + index) {
            return token.kind == kind;
        }

        false
    }

    fn advance(&mut self) {
        self.offset += 1;
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.offset)
    }

    fn unexpected_end_of_input(&self) -> ParsingError {
        let first_token_span = Span {
            start: Position { line: 1, col: 1 },
            end: Position { line: 1, col: 1 },
        };

        let last_token_span = self
            .tokens
            .last()
            .map(|t| &t.span)
            .unwrap_or(&first_token_span);

        ParsingError::new(
            ParsingErrorKind::UnexpectedEndOfInput,
            last_token_span.clone(),
        )
    }

    fn get_span(&mut self, start_offset: usize, end_offset: usize) -> Result<Span, ParsingError> {
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

    pub fn consume_string(&mut self) -> Result<StringNode, ParsingError> {
        if let Some(t) = self.current() {
            match t.clone() {
                Token {
                    kind: TokenKind::String(value),
                    span,
                } => {
                    self.advance();

                    Ok(StringNode { span, value })
                }
                t => {
                    return Err(ParsingError::new(
                        ParsingErrorKind::ExpectedAStringValue,
                        t.span,
                    ))
                }
            }
        } else {
            Err(self.unexpected_end_of_input())
        }
    }

    pub fn consume_punctuation(&mut self, expected: PunctuationKind) -> Result<(), ParsingError> {
        if let Some(token) = self.current() {
            match &token.kind {
                TokenKind::Punctuation(pk) if pk == &expected => {
                    self.advance();
                    Ok(())
                }
                _ => Err(ParsingError::new(
                    ParsingErrorKind::ExpectedAPunctuationMark(expected),
                    token.span,
                )),
            }
        } else {
            Err(self.unexpected_end_of_input())
        }
    }

    pub fn consume_number(&mut self) -> Result<NumberKind, ParsingError> {
        if let Some(token) = self.current() {
            match token.kind {
                TokenKind::Number(number_kind) => {
                    self.advance();
                    return Ok(number_kind);
                }
                _ => {
                    return Err(ParsingError::new(
                        ParsingErrorKind::ExpectedANumericValue,
                        token.span,
                    ))
                }
            }
        }

        Err(self.unexpected_end_of_input())
    }

    pub fn consume_keyword(&mut self, expected: KeywordKind) -> Result<(), ParsingError> {
        if let Some(token) = self.current() {
            match token.kind {
                TokenKind::Keyword(keyword_kind) if keyword_kind == expected => {
                    self.advance();
                    Ok(())
                }
                _ => Err(ParsingError::new(
                    ParsingErrorKind::ExpectedAKeyword(expected),
                    token.span,
                )),
            }
        } else {
            Err(self.unexpected_end_of_input())
        }
    }

    pub fn consume_identifier(&mut self) -> Result<IdentifierNode, ParsingError> {
        if let Some(token) = self.current() {
            match &token.kind {
                TokenKind::Identifier(id) => {
                    let identifier = id.clone();
                    let span = token.span;
                    self.advance();
                    Ok(IdentifierNode {
                        name: identifier,
                        span,
                    })
                }
                _ => Err(ParsingError::new(
                    ParsingErrorKind::ExpectedAnIdentifier,
                    token.span,
                )),
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
                message: doc.to_owned(),
                span: span.clone(),
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
    ) -> Result<Vec<T>, ParsingError>
    where
        F: FnMut(&mut Self) -> Result<T, ParsingError>,
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

    pub fn parse(tokens: Vec<Token>) -> Vec<Stmt> {
        let mut state = Parser {
            offset: 0,
            checkpoint_offset: 0,
            tokens,
        };

        let mut statements: Vec<Stmt> = vec![];

        while state.current().is_some() {
            let stmt = state.parse_stmt();
            let unwrapped = stmt.unwrap_or_else(|e| Stmt {
                span: e.span,
                kind: StmtKind::Error(e),
            });

            statements.push(unwrapped);
        }

        statements
    }
}
