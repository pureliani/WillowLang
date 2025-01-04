use super::{TokenizationError, Tokenizer};

impl Tokenizer {
    pub fn string(&mut self) -> Result<String, TokenizationError> {
        self.consume();
        let literal_start = self.offset;

        while let Some(c) = self.current() {
            match c {
                "\"" => {
                    let result = self.slice(literal_start, self.offset).to_owned();
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
                                return Err(TokenizationError::UnknownEscapeSequence);
                            }
                        }
                    } else {
                        return Err(TokenizationError::UnterminatedString);
                    }
                }
                _ => self.consume(),
            }
        }

        Err(TokenizationError::UnterminatedString)
    }
}
