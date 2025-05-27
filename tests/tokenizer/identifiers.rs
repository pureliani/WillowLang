use pretty_assertions::assert_eq;
use willow::{
    ast::{Position, Span},
    tokenize::{KeywordKind, Token, TokenKind, Tokenizer},
};

#[test]
fn tokenizes_simple_identifiers() {
    let (tokens, _) = Tokenizer::tokenize("hello");

    assert_eq!(
        tokens,
        vec![Token {
            kind: TokenKind::Identifier("hello".to_owned()),
            span: Span {
                start: Position {
                    line: 1,
                    col: 1,
                    byte_offset: 0
                },
                end: Position {
                    line: 1,
                    col: 6,
                    byte_offset: 5
                }
            }
        }]
    )
}

#[test]
fn tokenizes_keywords() {
    let (tokens, _) = Tokenizer::tokenize("struct");

    assert_eq!(
        tokens,
        vec![Token {
            kind: TokenKind::Keyword(KeywordKind::Struct),
            span: Span {
                start: Position {
                    line: 1,
                    col: 1,
                    byte_offset: 0
                },
                end: Position {
                    line: 1,
                    col: 7,
                    byte_offset: 6
                }
            }
        }]
    )
}

#[test]
fn tokenizes_sequence_as_identifier() {
    let (tokens, _) = Tokenizer::tokenize("\nstructhello");

    assert_eq!(
        tokens,
        vec![Token {
            kind: TokenKind::Identifier("structhello".to_owned()),
            span: Span {
                start: Position {
                    line: 2,
                    col: 1,
                    byte_offset: 1
                },
                end: Position {
                    line: 2,
                    col: 12,
                    byte_offset: 12
                }
            }
        }]
    )
}
