use willow::{
    ast::{Position, Span},
    tokenizer::{KeywordKind, NumberKind, PunctuationKind, Token, TokenKind, Tokenizer},
};

#[test]
fn test_skip_single_line_comment() {
    let input = "// This is a comment\nlet x = 10;".to_owned();
    let (tokens, _) = Tokenizer::tokenize(input).to_owned();

    assert_eq!(
        tokens,
        vec![
            Token {
                kind: TokenKind::Keyword(KeywordKind::Let),
                span: Span {
                    start: Position { line: 2, col: 1 },
                    end: Position { line: 2, col: 4 }
                }
            },
            Token {
                kind: TokenKind::Identifier("x".to_owned()),
                span: Span {
                    start: Position { line: 2, col: 5 },
                    end: Position { line: 2, col: 6 }
                }
            },
            Token {
                kind: TokenKind::Punctuation(PunctuationKind::Eq),
                span: Span {
                    start: Position { line: 2, col: 7 },
                    end: Position { line: 2, col: 8 }
                }
            },
            Token {
                kind: TokenKind::Number(NumberKind::I64(10)),
                span: Span {
                    start: Position { line: 2, col: 9 },
                    end: Position { line: 2, col: 11 }
                }
            },
            Token {
                kind: TokenKind::Punctuation(PunctuationKind::SemiCol),
                span: Span {
                    start: Position { line: 2, col: 11 },
                    end: Position { line: 2, col: 12 }
                }
            }
        ]
    );
}

#[test]
fn test_skip_multiple_single_line_comments() {
    let input = "// Comment 1\n// Comment 2\nlet x = 10;".to_owned();
    let (tokens, _) = Tokenizer::tokenize(input);

    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].kind, TokenKind::Keyword(KeywordKind::Let));
}

#[test]
fn test_comment_at_end_of_input() {
    let input = "let x = 10; // Comment at the end".to_owned();
    let (tokens, _) = Tokenizer::tokenize(input);

    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].kind, TokenKind::Keyword(KeywordKind::Let));
}

#[test]
fn test_no_comments() {
    let input = "let x = 10;".to_owned();
    let (tokens, _) = Tokenizer::tokenize(input);

    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].kind, TokenKind::Keyword(KeywordKind::Let));
}

#[test]
fn test_only_comments() {
    let input = "// Only a comment".to_owned();
    let (tokens, _) = Tokenizer::tokenize(input);
    assert_eq!(tokens.len(), 0);
}
