pub mod solns;

pub fn parse_decimal_to_integer_optimized(decimal_str: &str) -> i16 {
    let mut result = 0i16;
    let bytes = decimal_str.as_bytes();
    let mut start_index = 0;

    // Check if the string is negative
    if bytes.first() == Some(&b'-') {
        start_index = 1;
    }
    for &b in &bytes[start_index..bytes.len()] {
        if b.is_ascii_digit() {
            result = result * 10 + (b - b'0') as i16;
        }
    }

    if start_index == 1 {
        -result
    } else {
        result
    }
}

pub fn parse_decimal_v2(decimal_str: &str) -> i16 {
    let (temp_int, temp_dec) = decimal_str
        .split_once('.')
        .expect("{decimal_str} is not a decimal number");
    let temp_int: i16 = temp_int.parse().unwrap();
    let temp_dec: i16 = temp_dec.parse().unwrap();
    let mut temp: i16 = temp_int.abs() * 10 + temp_dec.abs();
    if temp_int.is_negative() {
        temp = -temp;
    }
    temp
}

//write me a test for the function parse_decimal_to_integer_optimized
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_decimal_to_integer_optimized() {
        assert_eq!(parse_decimal_to_integer_optimized("123"), 123);
        assert_eq!(parse_decimal_to_integer_optimized("-123"), -123);
        assert_eq!(parse_decimal_to_integer_optimized("0"), 0);
        assert_eq!(parse_decimal_to_integer_optimized("0.0"), 0);
    }
    #[test]
    fn test_parse_decimal_v2() {
        assert_eq!(parse_decimal_v2("123.4"), 1234);
        assert_eq!(parse_decimal_v2("-123.4"), -1234);
        assert_eq!(parse_decimal_v2("0.0"), 0);
    }
}
