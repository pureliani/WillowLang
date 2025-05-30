use unicode_segmentation::UnicodeSegmentation;

pub mod tokenize_documentation;
pub mod tokenize_identifier;
pub mod tokenize_number;
pub mod tokenize_punctuation;
pub mod tokenize_string;

use crate::ast::{Position, Span};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenizationErrorKind {
    UnknownToken,
    UnknownEscapeSequence,
    InvalidFloatingNumber,
    InvalidIntegerNumber,
    UnterminatedString,
    UnterminatedDoc,
}

impl TokenizationErrorKind {
    pub fn code(&self) -> usize {
        match self {
            TokenizationErrorKind::UnknownToken => 1,
            TokenizationErrorKind::UnknownEscapeSequence => 2,
            TokenizationErrorKind::InvalidFloatingNumber => 3,
            TokenizationErrorKind::InvalidIntegerNumber => 4,
            TokenizationErrorKind::UnterminatedString => 5,
            TokenizationErrorKind::UnterminatedDoc => 6,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenizationError {
    pub kind: TokenizationErrorKind,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PunctuationKind {
    DoubleCol,
    DoubleOr,
    DoubleAnd,
    DoubleEq,
    Col,
    SemiCol,
    Lt,
    Gt,
    Lte,
    Gte,
    Or,
    And,
    Not,
    Dot,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Eq,
    NotEq,
    Plus,
    Minus,
    Slash,
    Star,
    Percent,
    Comma,
    Dollar,
    Question,
    FatArrow,
}

impl PunctuationKind {
    pub fn to_string(&self) -> String {
        String::from(match self {
            PunctuationKind::DoubleCol => "::",
            PunctuationKind::DoubleOr => "||",
            PunctuationKind::DoubleAnd => "&&",
            PunctuationKind::DoubleEq => "==",
            PunctuationKind::Col => ":",
            PunctuationKind::SemiCol => ";",
            PunctuationKind::Lt => "<",
            PunctuationKind::Gt => ">",
            PunctuationKind::Lte => "<=",
            PunctuationKind::Gte => ">=",
            PunctuationKind::Or => "|",
            PunctuationKind::And => "&",
            PunctuationKind::Not => "!",
            PunctuationKind::Dot => ".",
            PunctuationKind::LParen => "(",
            PunctuationKind::RParen => ")",
            PunctuationKind::LBracket => "[",
            PunctuationKind::RBracket => "]",
            PunctuationKind::LBrace => "{",
            PunctuationKind::RBrace => "}",
            PunctuationKind::Eq => "=",
            PunctuationKind::NotEq => "!=",
            PunctuationKind::Plus => "+",
            PunctuationKind::Minus => "-",
            PunctuationKind::Slash => "/",
            PunctuationKind::Star => "*",
            PunctuationKind::Percent => "%",
            PunctuationKind::Comma => ",",
            PunctuationKind::Dollar => "$",
            PunctuationKind::Question => "?",
            PunctuationKind::FatArrow => "=>",
        })
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum KeywordKind {
    Struct,
    Enum,
    Let,
    Return,
    If,
    Else,
    While,
    Break,
    Continue,
    Type,
    From,
    Void,
    Null,
    True,
    False,
    Pub,
    Char,
    Bool,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    USize,
    ISize,
    F32,
    F64,
}

impl KeywordKind {
    pub fn to_string(&self) -> String {
        String::from(match self {
            KeywordKind::Struct => "struct",
            KeywordKind::Enum => "enum",
            KeywordKind::Let => "let",
            KeywordKind::Return => "return",
            KeywordKind::If => "if",
            KeywordKind::Else => "else",
            KeywordKind::While => "while",
            KeywordKind::Break => "break",
            KeywordKind::Continue => "continue",
            KeywordKind::Type => "type",
            KeywordKind::From => "from",
            KeywordKind::Void => "void",
            KeywordKind::Null => "null",
            KeywordKind::True => "true",
            KeywordKind::False => "false",
            KeywordKind::Pub => "pub",
            KeywordKind::Char => "char",
            KeywordKind::Bool => "bool",
            KeywordKind::I8 => "i8",
            KeywordKind::I16 => "i16",
            KeywordKind::I32 => "i32",
            KeywordKind::I64 => "i64",
            KeywordKind::U8 => "u8",
            KeywordKind::U16 => "u16",
            KeywordKind::U32 => "u32",
            KeywordKind::U64 => "u64",
            KeywordKind::USize => "uSize",
            KeywordKind::ISize => "iSize",
            KeywordKind::F32 => "f32",
            KeywordKind::F64 => "f64",
        })
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum NumberKind {
    I64(i64),
    I32(i32),
    I16(i16),
    I8(i8),
    F32(f32),
    F64(f64),
    U64(u64),
    U32(u32),
    U16(u16),
    U8(u8),
    USize(usize),
    ISize(isize),
}

impl NumberKind {
    pub fn to_string(&self) -> String {
        match self {
            NumberKind::I64(v) => format!("{}i64", v),
            NumberKind::I32(v) => format!("{}i32", v),
            NumberKind::I16(v) => format!("{}i16", v),
            NumberKind::I8(v) => format!("{}i8", v),
            NumberKind::F32(v) => format!("{}f32", v),
            NumberKind::F64(v) => format!("{}f64", v),
            NumberKind::U64(v) => format!("{}u64", v),
            NumberKind::U32(v) => format!("{}u32", v),
            NumberKind::U16(v) => format!("{}u16", v),
            NumberKind::U8(v) => format!("{}u8", v),
            NumberKind::USize(v) => format!("{}usize", v),
            NumberKind::ISize(v) => format!("{}isize", v),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind<'a> {
    Identifier(&'a str),
    Punctuation(PunctuationKind),
    Keyword(KeywordKind),
    String(&'a str),
    Number(NumberKind),
    Doc(&'a str),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token<'a> {
    pub span: Span,
    pub kind: TokenKind<'a>,
}

impl<'a> TokenKind<'a> {
    pub fn to_string(&self) -> &'a str {
        match self {
            TokenKind::Identifier(id) => id,
            TokenKind::Punctuation(punctuation_kind) => punctuation_kind.to_string(),
            TokenKind::Keyword(keyword_kind) => keyword_kind.to_string(),
            TokenKind::String(value) => &format!("\"{}\"", value),
            TokenKind::Number(number_kind) => number_kind.to_string(),
            TokenKind::Doc(doc) => doc,
        }
    }
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
    input: &'a str,
    byte_offset: usize,
    grapheme_offset: usize,
    line: usize,
    col: usize,
}

impl<'a> Tokenizer<'a> {
    fn current(&self) -> Option<&'a str> {
        self.input.graphemes(true).nth(self.grapheme_offset)
    }

    fn consume(&mut self) {
        if let Some(c) = self.current() {
            if c == "\n" {
                self.byte_offset += c.len();
                self.line += 1;
                self.col = 1;
            } else {
                self.byte_offset += c.len();
                self.col += 1;
            }
            self.grapheme_offset += 1;
        }
    }

    fn peek(&self, i: usize) -> Option<&'a str> {
        self.input.graphemes(true).nth(self.grapheme_offset + i)
    }

    fn slice(&self, start: usize, end: usize) -> &'a str {
        let grapheme_indices: Vec<(usize, &str)> = self.input.grapheme_indices(true).collect();

        let start_idx = grapheme_indices[start].0;
        let end_idx = if end < grapheme_indices.len() {
            grapheme_indices[end].0
        } else {
            self.input.len()
        };

        &self.input[start_idx..end_idx]
    }

    fn synchronize(&mut self) {
        while let Some(ch) = self.current() {
            let is_whitespace = ch.chars().all(|c| c.is_whitespace());

            if is_whitespace || ch == ";" || ch == "," {
                self.consume();
                break;
            } else {
                self.consume();
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current() {
            let is_whitespace = ch.chars().all(|c| c.is_whitespace());

            if is_whitespace {
                self.consume();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        if self.peek(0) == Some("/") && self.peek(1) == Some("/") {
            while let Some(c) = self.current() {
                if c == "\n" {
                    break;
                }
                self.consume();
            }
        }
    }

    pub fn tokenize(input: &'a str) -> (Vec<Token<'a>>, Vec<TokenizationError>) {
        let mut state = Tokenizer {
            input,
            byte_offset: 0,
            grapheme_offset: 0,
            line: 1,
            col: 1,
        };
        let mut tokens: Vec<Token<'a>> = vec![];
        let mut errors: Vec<TokenizationError> = vec![];

        loop {
            state.skip_whitespace();
            state.skip_comment();

            let start_pos = Position {
                line: state.line,
                col: state.col,
                byte_offset: state.byte_offset,
            };

            match state.current() {
                Some(letter) if is_letter(letter) => {
                    let identifier = state.tokenize_identifier();
                    let keyword = is_keyword(&identifier);
                    let kind = if let Some(keyword_kind) = keyword {
                        TokenKind::Keyword(keyword_kind)
                    } else {
                        TokenKind::Identifier(identifier)
                    };
                    let end_pos = Position {
                        line: state.line,
                        col: state.col,
                        byte_offset: state.byte_offset,
                    };

                    tokens.push(Token {
                        span: Span {
                            start: start_pos,
                            end: end_pos,
                        },
                        kind,
                    });
                }
                Some("\"") => match state.string() {
                    Ok(value) => {
                        let end_pos = Position {
                            line: state.line,
                            col: state.col,
                            byte_offset: state.byte_offset,
                        };
                        tokens.push(Token {
                            span: Span {
                                start: start_pos,
                                end: end_pos,
                            },
                            kind: TokenKind::String(value),
                        })
                    }
                    Err(kind) => {
                        let end_pos = Position {
                            line: state.line,
                            col: state.col,
                            byte_offset: state.byte_offset,
                        };
                        errors.push(TokenizationError {
                            kind,
                            span: Span {
                                start: start_pos,
                                end: end_pos,
                            },
                        });
                        state.synchronize();
                    }
                },
                Some(digit) if is_digit(digit) => match state.tokenize_number() {
                    Ok(number_kind) => {
                        let end_pos = Position {
                            line: state.line,
                            col: state.col,
                            byte_offset: state.byte_offset,
                        };
                        tokens.push(Token {
                            kind: TokenKind::Number(number_kind),
                            span: Span {
                                start: start_pos,
                                end: end_pos,
                            },
                        })
                    }
                    Err(kind) => {
                        let end_pos = Position {
                            line: state.line,
                            col: state.col,
                            byte_offset: state.byte_offset,
                        };
                        errors.push(TokenizationError {
                            kind,
                            span: Span {
                                start: start_pos,
                                end: end_pos,
                            },
                        });
                        state.synchronize();
                    }
                },
                Some("-") if state.peek(1) == Some("-") && state.peek(2) == Some("-") => {
                    match state.tokenize_documentation() {
                        Ok(content) => {
                            let end_pos = Position {
                                line: state.line,
                                col: state.col,
                                byte_offset: state.byte_offset,
                            };
                            tokens.push(Token {
                                kind: TokenKind::Doc(content),
                                span: Span {
                                    start: start_pos,
                                    end: end_pos,
                                },
                            })
                        }
                        Err(kind) => {
                            let end_pos = Position {
                                line: state.line,
                                col: state.col,
                                byte_offset: state.byte_offset,
                            };
                            errors.push(TokenizationError {
                                kind,
                                span: Span {
                                    start: start_pos,
                                    end: end_pos,
                                },
                            });
                            state.synchronize();
                        }
                    }
                }
                Some(punct) => match state.tokenize_punctuation(punct) {
                    Some(kind) => {
                        let end_pos = Position {
                            line: state.line,
                            col: state.col,
                            byte_offset: state.byte_offset,
                        };
                        tokens.push(Token {
                            kind: TokenKind::Punctuation(kind),
                            span: Span {
                                start: start_pos,
                                end: end_pos,
                            },
                        })
                    }
                    None => {
                        let end_pos = Position {
                            line: state.line,
                            col: state.col,
                            byte_offset: state.byte_offset,
                        };
                        errors.push(TokenizationError {
                            kind: TokenizationErrorKind::UnknownToken,
                            span: Span {
                                start: start_pos,
                                end: end_pos,
                            },
                        });
                        state.synchronize();
                    }
                },
                None => break,
            };
        }

        (tokens, errors)
    }
}

fn is_letter(value: &str) -> bool {
    value.graphemes(true).count() == 1 && value.chars().all(char::is_alphabetic)
}

fn is_digit(value: &str) -> bool {
    value.graphemes(true).count() == 1 && value.chars().all(|x| char::is_ascii_digit(&x))
}

fn is_alphanumeric(value: &str) -> bool {
    value.graphemes(true).count() == 1 && value.chars().all(char::is_alphanumeric)
}

fn is_keyword(identifier: &str) -> Option<KeywordKind> {
    match identifier {
        "struct" => Some(KeywordKind::Struct),
        "enum" => Some(KeywordKind::Enum),
        "let" => Some(KeywordKind::Let),
        "return" => Some(KeywordKind::Return),
        "if" => Some(KeywordKind::If),
        "else" => Some(KeywordKind::Else),
        "while" => Some(KeywordKind::While),
        "break" => Some(KeywordKind::Break),
        "continue" => Some(KeywordKind::Continue),
        "type" => Some(KeywordKind::Type),
        "from" => Some(KeywordKind::From),
        "void" => Some(KeywordKind::Void),
        "null" => Some(KeywordKind::Null),
        "true" => Some(KeywordKind::True),
        "false" => Some(KeywordKind::False),
        "pub" => Some(KeywordKind::Pub),
        "char" => Some(KeywordKind::Char),
        "bool" => Some(KeywordKind::Bool),
        "i8" => Some(KeywordKind::I8),
        "i16" => Some(KeywordKind::I16),
        "i32" => Some(KeywordKind::I32),
        "i64" => Some(KeywordKind::I64),
        "u8" => Some(KeywordKind::U8),
        "u16" => Some(KeywordKind::U16),
        "u32" => Some(KeywordKind::U32),
        "u64" => Some(KeywordKind::U64),
        "f32" => Some(KeywordKind::F32),
        "f64" => Some(KeywordKind::F64),
        "usize" => Some(KeywordKind::USize),
        "isize" => Some(KeywordKind::ISize),
        _ => None,
    }
}
