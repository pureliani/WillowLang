use super::{is_alphanumeric, Tokenizer};

impl<'a> Tokenizer<'a> {
    pub fn tokenize_identifier(&mut self) -> &'a str {
        let start = self.grapheme_offset;
        while let Some(c) = self.current() {
            if is_alphanumeric(c) || c == "_" {
                self.consume();
            } else {
                break;
            }
        }

        self.slice(start, self.grapheme_offset)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Position, Span},
        tokenize::{Token, TokenKind, Tokenizer},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn tokenizes_simple_identifiers() {
        let (tokens, _) = Tokenizer::tokenize("hello");

        assert_eq!(
            tokens,
            vec![Token {
                kind: TokenKind::Identifier("hello"),
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
    fn tokenizes_sequence_as_identifier() {
        let (tokens, _) = Tokenizer::tokenize("\nstructhello");

        assert_eq!(
            tokens,
            vec![Token {
                kind: TokenKind::Identifier("structhello"),
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
}
