use crate::{
    ast::{
        base::{
            base_expression::BlockContents,
            base_statement::{Stmt, StmtKind},
        },
        Span,
    },
    parse::{statements::is_start_of_stmt, Parser, ParsingError, ParsingErrorKind},
    tokenize::{PunctuationKind, TokenKind},
};

use super::is_start_of_expr;

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_codeblock_expr(&mut self) -> Result<BlockContents, ParsingError<'a>> {
        let start_offset = self.offset;
        self.consume_punctuation(PunctuationKind::LBrace)?;

        let mut statements = Vec::new();
        let mut final_expr = None;

        loop {
            if self.match_token(0, TokenKind::Punctuation(PunctuationKind::RBrace)) {
                break;
            }

            let current_token = self.current().ok_or_else(|| self.unexpected_end_of_input())?;
            let current_token_span = current_token.span;

            if is_start_of_stmt(&current_token.kind) {
                if final_expr.is_some() {
                    return Err(ParsingError {
                        kind: ParsingErrorKind::UnexpectedStatementAfterFinalExpression,
                        span: current_token_span,
                    });
                }

                let stmt = match self.parse_stmt() {
                    Ok(s) => s,
                    Err(e) => {
                        self.synchronize_stmt();
                        return Err(e);
                    }
                };
                statements.push(stmt);
                final_expr = None;
            } else if is_start_of_expr(&current_token.kind) {
                if final_expr.is_some() {
                    return Err(ParsingError {
                        kind: ParsingErrorKind::UnexpectedTokenAfterFinalExpression {
                            found: current_token.kind.clone(),
                        },
                        span: current_token_span,
                    });
                }

                let expr = self.parse_expr(0, true).map_err(|e| {
                    self.synchronize_expr();
                    e
                })?;

                if self.match_token(0, TokenKind::Punctuation(PunctuationKind::SemiCol)) {
                    let semi_offset = self.offset;
                    self.advance();
                    let end_span = self.get_span(semi_offset, self.offset - 1)?;

                    statements.push(Stmt {
                        span: Span {
                            start: expr.span.start,
                            end: end_span.end,
                        },
                        kind: StmtKind::Expression(expr),
                    });

                    final_expr = None;
                } else {
                    final_expr = Some(Box::new(expr));
                }
            } else {
                return Err(ParsingError {
                    kind: ParsingErrorKind::ExpectedStatementOrExpression {
                        found: current_token.kind.clone(),
                    },
                    span: current_token_span,
                });
            }

            if final_expr.is_some() && !self.match_token(0, TokenKind::Punctuation(PunctuationKind::RBrace)) {
                let unexpected_token = self.current().cloned().ok_or_else(|| self.unexpected_end_of_input())?;
                return Err(ParsingError {
                    kind: ParsingErrorKind::UnexpectedTokenAfterFinalExpression {
                        found: unexpected_token.kind,
                    },
                    span: unexpected_token.span,
                });
            }
        }

        self.consume_punctuation(PunctuationKind::RBrace)?;

        let span = self.get_span(start_offset, self.offset - 1)?;

        Ok(BlockContents {
            statements,
            final_expr,
            span,
        })
    }
}
