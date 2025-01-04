pub mod parse_codeblock_expr;
pub mod parse_fn_call_expr;
pub mod parse_fn_expr;
pub mod parse_if_expr;
pub mod parse_parenthesized_expr;
pub mod parse_struct_init_expr;

use crate::{
    ast::{
        base::base_expression::{Expr, ExprKind},
        Span,
    },
    tokenizer::{KeywordKind, PunctuationKind, Token, TokenKind},
};

use super::{Parser, ParsingError, ParsingErrorKind};

fn prefix_bp(token_kind: &TokenKind) -> Option<((), u8)> {
    use PunctuationKind::*;
    use TokenKind::*;

    let priority = match token_kind {
        Punctuation(Minus) | Punctuation(Not) => ((), 13),
        _ => return None,
    };

    Some(priority)
}

fn infix_bp(token_kind: &TokenKind) -> Option<(u8, u8)> {
    use PunctuationKind::*;
    use TokenKind::*;

    let priority = match token_kind {
        Punctuation(DoubleOr) => (1, 2),
        Punctuation(DoubleAnd) => (3, 4),
        Punctuation(DoubleEq) | Punctuation(NotEq) => (5, 6),
        Punctuation(Lt) | Punctuation(Lte) | Punctuation(Gt) | Punctuation(Gte) => (7, 8),
        Punctuation(Plus) | Punctuation(Minus) => (9, 10),
        Punctuation(Star) | Punctuation(Slash) | Punctuation(Percent) => (11, 12),
        _ => return None,
    };

    Some(priority)
}

fn suffix_bp(token_kind: &TokenKind) -> Option<(u8, ())> {
    use PunctuationKind::*;
    use TokenKind::*;

    let priority = match token_kind {
        Punctuation(LParen) | Punctuation(LBrace) => (14, ()), // fn call and struct init
        Punctuation(Dot) | Punctuation(DoubleCol) => (14, ()), // member/static accesses
        Punctuation(Lt) => (14, ()),                           // generic struct/fn call
        _ => return None,
    };

    Some(priority)
}

pub fn is_start_of_expr(token_kind: &TokenKind) -> bool {
    match token_kind {
        TokenKind::Identifier(_)
        | TokenKind::Number(_)
        | TokenKind::String(_)
        | TokenKind::Keyword(KeywordKind::True)
        | TokenKind::Keyword(KeywordKind::False)
        | TokenKind::Keyword(KeywordKind::Null)
        | TokenKind::Keyword(KeywordKind::If)               // if expressions
        | TokenKind::Punctuation(PunctuationKind::LParen)   // Parenthesized or fn expr
        | TokenKind::Punctuation(PunctuationKind::LBrace)   // Codeblock expr
        | TokenKind::Punctuation(PunctuationKind::LBracket) // Array literal
        | TokenKind::Punctuation(PunctuationKind::Lt)       // fn expression
        | TokenKind::Punctuation(PunctuationKind::Minus)    // Negation
        | TokenKind::Punctuation(PunctuationKind::And)      // Address-of
        | TokenKind::Punctuation(PunctuationKind::Not)      // Logical NOT
        => true,
        _ => false,
    }
}

impl Parser {
    pub fn parse_expr(&mut self, min_prec: u8) -> Result<Expr, ParsingError> {
        let token = self.current().ok_or(self.unexpected_end_of_input())?;

        let token_span = token.span;

        let mut lhs = match token {
            Token {
                kind: TokenKind::Identifier(_),
                ..
            } => {
                let id = self.consume_identifier()?;
                Expr {
                    kind: ExprKind::Identifier(id),
                    span: token_span.clone(),
                }
            }
            Token {
                kind: TokenKind::Number(_),
                ..
            } => {
                let number = self.consume_number()?;
                Expr {
                    kind: ExprKind::Number { value: number },
                    span: token_span.clone(),
                }
            }
            Token {
                kind: TokenKind::Punctuation(PunctuationKind::Lt),
                ..
            } => self.parse_fn_expr()?,
            Token {
                kind: TokenKind::Punctuation(PunctuationKind::LParen),
                ..
            } => {
                self.place_checkpoint();
                let result = self.parse_fn_expr().or_else(|_| {
                    self.goto_checkpoint();
                    self.parse_parenthesized_expr()
                    // TODO: report an error when all parsing attempts fail
                })?;

                result
            }
            Token {
                kind: TokenKind::Punctuation(PunctuationKind::LBrace),
                ..
            } => {
                let start_offset = self.offset;

                let block_contents = self.parse_codeblock_expr()?;

                Expr {
                    kind: ExprKind::Block(block_contents),
                    span: self.get_span(start_offset, self.offset - 1)?,
                }
            }
            Token {
                kind: TokenKind::Punctuation(PunctuationKind::LBracket),
                ..
            } => {
                let start_offset = self.offset;
                self.consume_punctuation(PunctuationKind::LBracket)?;
                let items: Vec<Box<Expr>> = self
                    .comma_separated(
                        |p| p.parse_expr(0),
                        |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::RBracket)),
                    )?
                    .into_iter()
                    .map(|item| Box::new(item))
                    .collect();

                self.consume_punctuation(PunctuationKind::RBracket)?;
                let span = self.get_span(start_offset, self.offset - 1)?;

                Expr {
                    kind: ExprKind::ArrayLiteral { items },
                    span,
                }
            }
            Token {
                kind: TokenKind::Punctuation(PunctuationKind::Minus),
                ..
            } => {
                let ((), r_bp) =
                    prefix_bp(&TokenKind::Punctuation(PunctuationKind::Minus)).unwrap();
                let start_offset = self.offset;

                self.consume_punctuation(PunctuationKind::Minus)?;
                let expr = self.parse_expr(r_bp)?;
                Expr {
                    kind: ExprKind::Neg {
                        right: Box::new(expr),
                    },
                    span: self.get_span(start_offset, self.offset - 1)?,
                }
            }
            Token {
                kind: TokenKind::Punctuation(PunctuationKind::Not),
                ..
            } => {
                let ((), r_bp) = prefix_bp(&TokenKind::Punctuation(PunctuationKind::Not)).unwrap();
                let start_offset = self.offset;

                self.consume_punctuation(PunctuationKind::Not)?;
                let expr = self.parse_expr(r_bp)?;
                Expr {
                    kind: ExprKind::Not {
                        right: Box::new(expr),
                    },
                    span: self.get_span(start_offset, self.offset - 1)?,
                }
            }
            Token {
                kind: TokenKind::Keyword(KeywordKind::If),
                ..
            } => self.parse_if_expr()?,
            Token {
                kind: TokenKind::Keyword(KeywordKind::True),
                ..
            } => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::True)?;
                Expr {
                    kind: ExprKind::BoolLiteral { value: true },
                    span: self.get_span(start_offset, self.offset - 1)?,
                }
            }
            Token {
                kind: TokenKind::Keyword(KeywordKind::False),
                ..
            } => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::False)?;

                Expr {
                    kind: ExprKind::BoolLiteral { value: false },
                    span: self.get_span(start_offset, self.offset - 1)?,
                }
            }
            Token {
                kind: TokenKind::Keyword(KeywordKind::Null),
                ..
            } => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::Null)?;
                Expr {
                    kind: ExprKind::Null,
                    span: self.get_span(start_offset, self.offset - 1)?,
                }
            }
            Token {
                kind: TokenKind::String(_),
                ..
            } => {
                let start_offset = self.offset;

                let val = self.consume_string()?;
                Expr {
                    kind: ExprKind::String(val),
                    span: self.get_span(start_offset, self.offset - 1)?,
                }
            }
            t => {
                return Err(ParsingError::new(
                    ParsingErrorKind::ExpectedAnExpressionButFound(t.clone()),
                    t.span,
                ))
            }
        };

        loop {
            let op = match self.current() {
                Some(o) => o.clone(),
                None => break,
            };

            if let Some((left_prec, ())) = suffix_bp(&op.kind) {
                if left_prec < min_prec {
                    break;
                }
                let lhs_clone = lhs.clone();

                let new_lhs = match op.kind {
                    TokenKind::Punctuation(PunctuationKind::Dot) => {
                        let start_offset = self.offset;

                        self.consume_punctuation(PunctuationKind::Dot)?;
                        let field = self.consume_identifier()?;
                        Some(Expr {
                            kind: ExprKind::Access {
                                left: Box::new(lhs_clone),
                                field: field,
                            },
                            span: self.get_span(start_offset, self.offset - 1)?,
                        })
                    }
                    TokenKind::Punctuation(PunctuationKind::DoubleCol) => {
                        let start_offset = self.offset;

                        self.consume_punctuation(PunctuationKind::DoubleCol)?;
                        let field = self.consume_identifier()?;
                        Some(Expr {
                            kind: ExprKind::StaticAccess {
                                left: Box::new(lhs_clone),
                                field,
                            },
                            span: self.get_span(start_offset, self.offset - 1)?,
                        })
                    }
                    TokenKind::Punctuation(PunctuationKind::Lt) => {
                        self.place_checkpoint();

                        if let Ok((generic_args, generic_args_span)) =
                            self.parse_optional_generic_args()
                        {
                            Some(Expr {
                                kind: ExprKind::GenericApply {
                                    left: Box::new(lhs_clone),
                                    args: generic_args,
                                },
                                span: generic_args_span,
                            })
                        } else {
                            self.goto_checkpoint();
                            None
                        }
                    }
                    TokenKind::Punctuation(PunctuationKind::LParen) => {
                        if let ExprKind::StaticAccess { left, field } = lhs.kind.clone() {
                            if field.name == "as" || field.name == "is" {
                                let start_offset = self.offset;
                                self.consume_punctuation(PunctuationKind::LParen)?;
                                let target_type = self.parse_type_annotation(0)?;
                                self.consume_punctuation(PunctuationKind::RParen)?;
                                let span_end = self.get_span(start_offset, self.offset - 1)?;

                                let kind = if field.name == "as" {
                                    ExprKind::TypeCast {
                                        left,
                                        target: target_type,
                                    }
                                } else {
                                    ExprKind::IsType {
                                        left,
                                        target: target_type,
                                    }
                                };

                                Some(Expr {
                                    kind,
                                    span: Span {
                                        start: lhs.span.start,
                                        end: span_end.end,
                                    },
                                })
                            } else {
                                return Err(ParsingError::new(
                                    ParsingErrorKind::UnknownStaticMethod(field.to_owned()),
                                    field.span,
                                ));
                            }
                        } else {
                            Some(self.parse_fn_call_expr(lhs_clone)?)
                        }
                    }
                    TokenKind::Punctuation(PunctuationKind::LBrace) => {
                        Some(self.parse_struct_init_expr(lhs_clone)?)
                    }
                    _ => {
                        return Err(ParsingError::new(
                            ParsingErrorKind::InvalidSuffixOperator(op.clone()),
                            op.span,
                        ))
                    }
                };

                if let Some(expr) = new_lhs {
                    lhs = expr;
                    continue;
                }
            }

            if let Some((left_prec, right_prec)) = infix_bp(&op.kind) {
                if left_prec < min_prec {
                    break;
                }

                let start_pos = lhs.span.start;

                self.advance();

                let rhs = self.parse_expr(right_prec)?;

                let end_pos = rhs.span.end;

                let expr_kind = match op.kind {
                    TokenKind::Punctuation(PunctuationKind::Plus) => ExprKind::Add {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    },
                    TokenKind::Punctuation(PunctuationKind::Minus) => ExprKind::Subtract {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    },
                    TokenKind::Punctuation(PunctuationKind::Star) => ExprKind::Multiply {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    },
                    TokenKind::Punctuation(PunctuationKind::Slash) => ExprKind::Divide {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    },
                    TokenKind::Punctuation(PunctuationKind::Percent) => ExprKind::Modulo {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    },
                    TokenKind::Punctuation(PunctuationKind::Lt) => ExprKind::LessThan {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    },
                    TokenKind::Punctuation(PunctuationKind::Lte) => ExprKind::LessThanOrEqual {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    },
                    TokenKind::Punctuation(PunctuationKind::Gt) => ExprKind::GreaterThan {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    },
                    TokenKind::Punctuation(PunctuationKind::Gte) => ExprKind::GreaterThanOrEqual {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    },
                    TokenKind::Punctuation(PunctuationKind::DoubleEq) => ExprKind::Equal {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    },
                    TokenKind::Punctuation(PunctuationKind::NotEq) => ExprKind::NotEqual {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    },
                    TokenKind::Punctuation(PunctuationKind::DoubleAnd) => ExprKind::And {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    },
                    TokenKind::Punctuation(PunctuationKind::DoubleOr) => ExprKind::Or {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    },
                    _ => break,
                };

                lhs = Expr {
                    kind: expr_kind,
                    span: Span {
                        start: start_pos,
                        end: end_pos,
                    },
                };

                continue;
            }

            break;
        }

        Ok(lhs)
    }

    fn synchronize_expr(&mut self) {
        loop {
            match self.current() {
                Some(token) => {
                    if is_start_of_expr(&token.kind) {
                        return;
                    }
                    if token.kind == TokenKind::Punctuation(PunctuationKind::SemiCol) {
                        self.advance();
                        return;
                    }
                    if token.kind == TokenKind::Punctuation(PunctuationKind::RBrace) {
                        self.advance();
                        return;
                    }
                    if token.kind == TokenKind::Punctuation(PunctuationKind::RParen) {
                        self.advance();
                        return;
                    }
                    if token.kind == TokenKind::Punctuation(PunctuationKind::RBracket) {
                        self.advance();
                        return;
                    }
                    if token.kind == TokenKind::Punctuation(PunctuationKind::Comma) {
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
