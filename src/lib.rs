use std::{error::Error, fmt::Display};

use regex::Regex;

const ZERO: &str = "0";
const MINUS: char = '-';
const CHARSET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const CHARSET_LENGTH: i64 = 36;
const CHARSET_REGEX: &str = "^-?[0-9A-Z]+$";

#[derive(Debug)]
pub enum DecodeError {
    InvalidBase36String,
    InvalidCharacter(char),
    CannotDecodeNegativeSign,
}

impl Error for DecodeError {}

impl Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidBase36String => write!(f, "Invalid base36 string"),
            Self::InvalidCharacter(c) => write!(f, "Invalid character found: {c}"),
            Self::CannotDecodeNegativeSign => write!(f, "Cannot decode negative sign"),
        }
    }
}

pub struct Base36 {}

impl Base36 {
    pub fn encode(number: i32) -> String {
        let mut number = i64::from(number);

        let is_negative = number < 0;
        if is_negative {
            number = -number;
        }

        if number == 0 {
            return String::from(ZERO);
        }

        let mut base_36 = Vec::new();

        while number > 0 {
            let char = CHARSET
                .chars()
                .nth((number % CHARSET_LENGTH) as usize)
                .unwrap();

            base_36.push(char);
            number /= CHARSET_LENGTH;
        }

        let mut base_36 = base_36.into_iter().rev().collect::<String>();

        if is_negative {
            base_36.insert(0, MINUS);
        }

        base_36
    }

    pub fn decode(base_36: String) -> Result<i32, DecodeError> {
        if !Regex::new(CHARSET_REGEX).unwrap().is_match(&base_36) {
            return Err(DecodeError::InvalidBase36String);
        }

        let mut base_36 = base_36;

        let is_negative = base_36.starts_with(MINUS);

        if is_negative {
            base_36 = base_36
                .get(1..)
                .ok_or(DecodeError::CannotDecodeNegativeSign)?
                .to_string();
        }

        let base_36 = base_36.chars().rev().collect::<String>();
        let mut num: i64 = 0;

        for (i, c) in base_36.chars().enumerate() {
            let position = CHARSET.find(c).ok_or(DecodeError::InvalidCharacter(c))? as i64;
            let power = CHARSET_LENGTH.pow(i as u32);

            num += position * power;
        }

        if is_negative {
            num = -num;
        }

        Ok(num as i32)
    }
}

#[cfg(test)]
mod tests {
    use crate::Base36;

    #[test]
    fn test_encode_zero() {
        let actual = Base36::encode(0);

        assert_eq!(actual, "0");
    }

    #[test]
    fn test_encode_positive_number() {
        let actual = Base36::encode(12345);

        assert_eq!(actual, "9IX");
    }

    #[test]
    fn test_encode_negative_number() {
        let actual = Base36::encode(-9876);

        assert_eq!(actual, "-7MC");
    }

    #[test]
    fn test_encode_large_number() {
        let actual: String = Base36::encode(987654321);

        assert_eq!(actual, "GC0UY9");
    }

    #[test]
    fn test_encode_max_i32() {
        let actual: String = Base36::encode(i32::MAX);

        assert_eq!(actual, "ZIK0ZJ");
    }

    #[test]
    fn test_encode_min_i32() {
        let actual = Base36::encode(i32::MIN);

        assert_eq!(actual, "-ZIK0ZK");
    }

    #[test]
    fn test_decode_error_invalid_base36() {
        let actual = Base36::decode(String::from("&/98@"));

        assert!(actual.is_err());
        assert_eq!(actual.err().unwrap().to_string(), "Invalid base36 string");
    }

    #[test]
    fn test_decode_zero() {
        let actual = Base36::decode(String::from("0")).unwrap();

        assert_eq!(actual, 0);
    }

    #[test]
    fn test_decode_positive_number() {
        let actual = Base36::decode(String::from("9IX")).unwrap();

        assert_eq!(actual, 12345);
    }

    #[test]
    fn test_decode_negative_number() {
        let actual = Base36::decode(String::from("-7MC")).unwrap();

        assert_eq!(actual, -9876);
    }

    #[test]
    fn test_decode_large_number() {
        let actual = Base36::decode(String::from("GC0UY9")).unwrap();

        assert_eq!(actual, 987654321);
    }

    #[test]
    fn test_decode_max_i32() {
        let actual = Base36::decode(String::from("ZIK0ZJ")).unwrap();

        assert_eq!(actual, i32::MAX);
    }

    #[test]
    fn test_decode_min_i32() {
        let actual = Base36::decode(String::from("-ZIK0ZK")).unwrap();

        assert_eq!(actual, i32::MIN);
    }
}
