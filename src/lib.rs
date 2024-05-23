pub struct Base36 {}

const ZERO: &str = "0";
const MINUS: char = '-';
const CHARSET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const CHARSET_LENGTH: i32 = 36;

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
                .nth((number % CHARSET_LENGTH as i64) as usize)
                .unwrap();

            base_36.push(char);
            number /= CHARSET_LENGTH as i64;
        }

        let mut base_36 = base_36.into_iter().rev().collect::<String>();

        if is_negative {
            base_36.insert(0, MINUS);
        }

        base_36
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
}
