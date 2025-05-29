use super::{TokenizationErrorKind, Tokenizer};

impl<'a> Tokenizer<'a> {
    pub fn string(&mut self) -> Result<&'a str, TokenizationErrorKind> {
        self.consume();
        let literal_start = self.grapheme_offset;

        while let Some(c) = self.current() {
            match c {
                "\"" => {
                    let value = self.slice(literal_start, self.grapheme_offset);
                    self.consume();
                    return Ok(value);
                }
                "\\" => {
                    self.consume();
                    if let Some(next_char) = self.current() {
                        match next_char {
                            "\"" | "\\" | "$" | "{" | "}" | "n" | "r" | "t" => {
                                self.consume();
                            }
                            _ => {
                                return Err(TokenizationErrorKind::UnknownEscapeSequence);
                            }
                        }
                    } else {
                        return Err(TokenizationErrorKind::UnterminatedString);
                    }
                }
                _ => self.consume(),
            }
        }

        Err(TokenizationErrorKind::UnterminatedString)
    }
}
