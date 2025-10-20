pub mod parse_assignment_stmt;
pub mod parse_break_stmt;
pub mod parse_continue_stmt;
pub mod parse_expr_stmt;
pub mod parse_from_stmt;
pub mod parse_return_stmt;
pub mod parse_type_alias_decl;
pub mod parse_var_decl;
pub mod parse_while_stmt;

use crate::{
    ast::{
        stmt::{Stmt, StmtKind},
        Span,
    },
    parse::{Parser, ParsingErrorKind},
    tokenize::{KeywordKind, PunctuationKind, TokenKind},
};

use super::ParsingError;

pub fn is_start_of_stmt(token_kind: &TokenKind) -> bool {
    matches!(
        token_kind,
        TokenKind::Keyword(
            KeywordKind::From
                | KeywordKind::While
                | KeywordKind::Return
                | KeywordKind::Break
                | KeywordKind::Continue
                | KeywordKind::Type
                | KeywordKind::Let
        ) | TokenKind::Doc(_)
    )
}

impl Parser {
    pub fn parse_stmt(&mut self) -> Result<Stmt, ParsingError> {
        let result = if self.match_token(0, TokenKind::Keyword(KeywordKind::From)) {
            self.parse_from_stmt()
        } else if self.match_token(0, TokenKind::Keyword(KeywordKind::While)) {
            self.parse_while_stmt()
        } else if self.match_token(0, TokenKind::Keyword(KeywordKind::Return)) {
            self.parse_return_stmt()
        } else if self.match_token(0, TokenKind::Keyword(KeywordKind::Break)) {
            self.parse_break_stmt()
        } else if self.match_token(0, TokenKind::Keyword(KeywordKind::Continue)) {
            self.parse_continue_stmt()
        } else {
            let documentation = self.consume_optional_doc();

            if self.match_token(0, TokenKind::Keyword(KeywordKind::Let)) {
                self.parse_var_decl(documentation)
            } else if self.match_token(0, TokenKind::Keyword(KeywordKind::Type)) {
                self.parse_type_alias_decl(documentation)
            } else if let Some(doc) = documentation {
                Err(ParsingError {
                    kind: ParsingErrorKind::DocMustBeFollowedByDeclaration,
                    span: doc.span,
                })
            } else {
                let lhs = self.parse_expr(0);

                match lhs {
                    Ok(lhs) => {
                        if self
                            .match_token(0, TokenKind::Punctuation(PunctuationKind::Eq))
                            && !self.match_token(
                                1,
                                TokenKind::Punctuation(PunctuationKind::Eq),
                            )
                        {
                            // It's an assignment statement
                            self.parse_assignment_stmt(lhs)
                        } else {
                            // It's a standalone expression statement
                            let mut end_span = lhs.span;
                            if self.match_token(
                                0,
                                TokenKind::Punctuation(PunctuationKind::SemiCol),
                            ) {
                                end_span = self.current().unwrap().span;
                                self.advance();
                            }

                            Ok(Stmt {
                                span: Span {
                                    start: lhs.span.start,
                                    end: end_span.end,
                                },
                                kind: StmtKind::Expression(lhs),
                            })
                        }
                    }
                    Err(e) => Err(e),
                }
            }
        };

        result.map_err(|e| {
            self.synchronize_stmt();
            e
        })
    }

    pub fn synchronize_stmt(&mut self) {
        loop {
            match self.current() {
                Some(token) => {
                    if token.kind == TokenKind::Punctuation(PunctuationKind::SemiCol) {
                        self.advance();
                        return;
                    }

                    self.advance();
                }
                None => return,
            }
        }
    }
}
