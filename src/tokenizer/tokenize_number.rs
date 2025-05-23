use super::{is_digit, is_letter, NumberKind, TokenizationErrorKind, Tokenizer};

impl<'a> Tokenizer<'a> {
    pub fn tokenize_number(&mut self) -> Result<NumberKind, TokenizationErrorKind> {
        let start = self.grapheme_offset;
        let mut has_dot = false;

        while let Some(c) = self.current() {
            if is_digit(c) {
                self.consume();
            } else if c == "." && !has_dot {
                has_dot = true;
                self.consume();
            } else if c == "." && has_dot {
                return Err(TokenizationErrorKind::InvalidFloatingNumber);
            } else if is_letter(c) {
                self.consume();
            } else {
                break;
            }
        }

        let number_str = self.slice(start, self.grapheme_offset);

        parse_number(number_str)
    }
}

fn parse_number(number: &str) -> Result<NumberKind, TokenizationErrorKind> {
    if number.contains('.') {
        if let Ok(value) = number.parse::<f64>() {
            return Ok(NumberKind::F64(value));
        }
        if let Ok(value) = number.parse::<f32>() {
            return Ok(NumberKind::F32(value));
        }
        return Err(TokenizationErrorKind::InvalidFloatingNumber);
    } else {
        if let Ok(value) = number.parse::<i64>() {
            return Ok(NumberKind::I64(value));
        }
        if let Ok(value) = number.parse::<i32>() {
            return Ok(NumberKind::I32(value));
        }
        if let Ok(value) = number.parse::<i16>() {
            return Ok(NumberKind::I16(value));
        }
        if let Ok(value) = number.parse::<i8>() {
            return Ok(NumberKind::I8(value));
        }
        if let Ok(value) = number.parse::<u64>() {
            return Ok(NumberKind::U64(value));
        }
        if let Ok(value) = number.parse::<u32>() {
            return Ok(NumberKind::U32(value));
        }
        if let Ok(value) = number.parse::<u16>() {
            return Ok(NumberKind::U16(value));
        }
        if let Ok(value) = number.parse::<u8>() {
            return Ok(NumberKind::U8(value));
        }
        if let Ok(value) = number.parse::<usize>() {
            return Ok(NumberKind::USize(value));
        }
        if let Ok(value) = number.parse::<isize>() {
            return Ok(NumberKind::ISize(value));
        }

        return Err(TokenizationErrorKind::InvalidIntegerNumber);
    }
}
