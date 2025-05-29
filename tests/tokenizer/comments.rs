use pretty_assertions::assert_eq;
use willow::{
    ast::{Position, Span},
    tokenize::{KeywordKind, NumberKind, PunctuationKind, Token, TokenKind, Tokenizer},
};

#[test]
fn test_skip_single_line_comment() {
    let input = "// This is a comment\nlet x = 10;";
    let (tokens, _) = Tokenizer::tokenize(input);

    assert_eq!(
        tokens,
        vec![
            Token {
                kind: TokenKind::Keyword(KeywordKind::Let),
                span: Span {
                    start: Position {
                        line: 2,
                        col: 1,
                        byte_offset: 21
                    },
                    end: Position {
                        line: 2,
                        col: 4,
                        byte_offset: 24
                    }
                }
            },
            Token {
                kind: TokenKind::Identifier("x"),
                span: Span {
                    start: Position {
                        line: 2,
                        col: 5,
                        byte_offset: 25
                    },
                    end: Position {
                        line: 2,
                        col: 6,
                        byte_offset: 26
                    }
                }
            },
            Token {
                kind: TokenKind::Punctuation(PunctuationKind::Eq),
                span: Span {
                    start: Position {
                        line: 2,
                        col: 7,
                        byte_offset: 27
                    },
                    end: Position {
                        line: 2,
                        col: 8,
                        byte_offset: 28
                    }
                }
            },
            Token {
                kind: TokenKind::Number(NumberKind::I64(10)),
                span: Span {
                    start: Position {
                        line: 2,
                        col: 9,
                        byte_offset: 29
                    },
                    end: Position {
                        line: 2,
                        col: 11,
                        byte_offset: 31
                    }
                }
            },
            Token {
                kind: TokenKind::Punctuation(PunctuationKind::SemiCol),
                span: Span {
                    start: Position {
                        line: 2,
                        col: 11,
                        byte_offset: 31
                    },
                    end: Position {
                        line: 2,
                        col: 12,
                        byte_offset: 32
                    }
                }
            }
        ]
    );
}

#[test]
fn test_skip_multiple_single_line_comments() {
    let input = "// Comment 1\n// Comment 2\nlet x = 10;";
    let (tokens, _) = Tokenizer::tokenize(input);

    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].kind, TokenKind::Keyword(KeywordKind::Let));
}

#[test]
fn test_comment_at_end_of_input() {
    let input = "let x = 10; // Comment at the end";
    let (tokens, _) = Tokenizer::tokenize(input);

    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].kind, TokenKind::Keyword(KeywordKind::Let));
}

#[test]
fn test_no_comments() {
    let input = "let x = 10;";
    let (tokens, _) = Tokenizer::tokenize(input);

    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].kind, TokenKind::Keyword(KeywordKind::Let));
}

#[test]
fn test_only_comments() {
    let input = "// Only a comment";
    let (tokens, _) = Tokenizer::tokenize(input);
    assert_eq!(tokens.len(), 0);
}
