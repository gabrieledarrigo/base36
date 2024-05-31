use std::{error::Error, fmt::Display};

const ZERO: &str = "0";
const MINUS: char = '-';
const CHARSET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const CHARSET_LENGTH: i64 = 36;

#[derive(Debug)]
pub enum ParsingError {
    InvalidIndex(usize),
}

impl Error for ParsingError {}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidIndex(i) => write!(f, "Invalid index: {i}"),
        }
    }
}

pub struct Base36 {
    value: String,
}

impl Base36 {
    fn new(value: String) -> Self {
        Self { value }
    }

    pub fn from(number: i32) -> Result<Base36, ParsingError> {
        let mut number = i64::from(number);

        let is_negative = number < 0;
        if is_negative {
            number = -number;
        }

        if number == 0 {
            return Ok(Base36::new(String::from(ZERO)));
        }

        let mut base_36 = Vec::new();

        while number > 0 {
            let index = (number % CHARSET_LENGTH) as usize;
            let char = CHARSET
                .chars()
                .nth(index)
                .ok_or(ParsingError::InvalidIndex(index))?;

            base_36.push(char);
            number /= CHARSET_LENGTH;
        }

        let mut base_36 = base_36.into_iter().rev().collect::<String>();

        if is_negative {
            base_36.insert(0, MINUS);
        }

        Ok(Base36::new(base_36))
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Display for Base36 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::Base36;

    #[test]
    fn test_from_zero() {
        let actual = Base36::from(0).unwrap();

        assert_eq!(actual.value(), "0");
    }

    #[test]
    fn test_from_positive_number() {
        let actual = Base36::from(12345).unwrap();

        assert_eq!(actual.value(), "9IX");
    }

    #[test]
    fn test_from_negative_number() {
        let actual = Base36::from(-9876).unwrap();

        assert_eq!(actual.value(), "-7MC");
    }

    #[test]
    fn test_from_large_number() {
        let actual = Base36::from(987654321).unwrap();

        assert_eq!(actual.value(), "GC0UY9");
    }

    #[test]
    fn test_from_max_i32() {
        let actual = Base36::from(i32::MAX).unwrap();

        assert_eq!(actual.value(), "ZIK0ZJ");
    }

    #[test]
    fn test_from_min_i32() {
        let actual = Base36::from(i32::MIN).unwrap();

        assert_eq!(actual.value(), "-ZIK0ZK");
    }
}
