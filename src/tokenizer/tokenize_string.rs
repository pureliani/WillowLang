use super::{TokenizationErrorKind, Tokenizer};

impl<'a> Tokenizer<'a> {
    pub fn string(&mut self) -> Result<String, TokenizationErrorKind> {
        self.consume();
        let literal_start = self.grapheme_offset;

        while let Some(c) = self.current() {
            match c {
                "\"" => {
                    let result = self.slice(literal_start, self.grapheme_offset).to_owned();
                    self.consume();
                    return Ok(result);
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
