pub mod parse_assignment_stmt;
pub mod parse_break_stmt;
pub mod parse_continue_stmt;
pub mod parse_enum_decl;
pub mod parse_expr_stmt;
pub mod parse_from_stmt;
pub mod parse_return_stmt;
pub mod parse_struct_decl;
pub mod parse_type_alias_decl;
pub mod parse_var_decl;
pub mod parse_while_stmt;

use crate::{
    ast::base::base_statement::Stmt,
    parse::{Parser, ParsingErrorKind},
    tokenize::{KeywordKind, PunctuationKind, TokenKind},
};

use super::{expressions::is_start_of_expr, ParsingError};

pub fn is_start_of_stmt(token_kind: &TokenKind) -> bool {
    match token_kind {
        TokenKind::Keyword(KeywordKind::From)
        | TokenKind::Keyword(KeywordKind::While)
        | TokenKind::Keyword(KeywordKind::Return)
        | TokenKind::Keyword(KeywordKind::Break)
        | TokenKind::Keyword(KeywordKind::Continue)
        | TokenKind::Keyword(KeywordKind::Struct)
        | TokenKind::Keyword(KeywordKind::Type)
        | TokenKind::Keyword(KeywordKind::Let)
        | TokenKind::Doc(_) => true,
        _ => false,
    }
}

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_stmt(&mut self) -> Result<Stmt, ParsingError<'a>> {
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

            if self.match_token(0, TokenKind::Keyword(KeywordKind::Struct)) {
                self.parse_struct_decl(documentation)
            } else if self.match_token(0, TokenKind::Keyword(KeywordKind::Enum)) {
                self.parse_enum_decl(documentation)
            } else if self.match_token(0, TokenKind::Keyword(KeywordKind::Type)) {
                self.parse_type_alias_decl(documentation)
            } else if self.match_token(0, TokenKind::Keyword(KeywordKind::Let)) {
                self.parse_var_decl(documentation)
            } else if let Some(doc) = documentation {
                Err(ParsingError {
                    kind: ParsingErrorKind::DocMustBeFollowedByDeclaration,
                    span: doc.span,
                })
            } else {
                let lhs = self.parse_expr(0, true);

                match lhs {
                    Ok(lhs) => {
                        if self.match_token(0, TokenKind::Punctuation(PunctuationKind::Eq)) {
                            // it's an assignment statement
                            self.parse_assignment_stmt(lhs)
                        } else {
                            // It's a standalone expression statement
                            self.parse_expr_stmt(lhs)
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
                    if is_start_of_stmt(&token.kind) || is_start_of_expr(&token.kind) {
                        return;
                    }
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
