/// Implement the myAtoi(string s) function, which converts a string to a 32-bit signed integer
/// (similar to C/C++'s atoi function).
///
/// The algorithm for myAtoi(string s) is as follows:
///
/// Read in and ignore any leading whitespace.
/// Check if the next character (if not already at the end of the string) is '-' or '+'.
/// Read this character in if it is either. This determines if the final result
/// is negative or positive respectively. Assume the result is positive if neither is present.
/// Read in next the characters until the next non-digit character or the end of the input is reached.
/// The rest of the string is ignored.
/// Convert these digits into an integer (i.e. "123" -> 123, "0032" -> 32). If no digits were read,
/// then the integer is 0. Change the sign as necessary (from step 2).
/// If the integer is out of the 32-bit signed integer range [-231, 231 - 1], then clamp the integer
/// so that it remains in the range. Specifically, integers less than -231 should be clamped to -231,
/// and integers greater than 231 - 1 should be clamped to 231 - 1.
/// Return the integer as the final result.
/// Note:
///
/// Only the space character ' ' is considered a whitespace character.
/// Do not ignore any characters other than the leading whitespace or the rest of the string after the digits.
///
/// Example 1:
/// Input: s = "42"
/// Output: 42
/// Explanation: The underlined characters are what is read in, the caret is the current reader position.
/// Step 1: "42" (no characters read because there is no leading whitespace)
///          ^
/// Step 2: "42" (no characters read because there is neither a '-' nor '+')
///          ^
/// Step 3: "42" ("42" is read in)
///            ^
/// The parsed integer is 42.
/// Since 42 is in the range [-231, 231 - 1], the final result is 42.
///
/// Example 2:
/// Input: s = "   -42"
/// Output: -42
/// Explanation:
/// Step 1: "   -42" (leading whitespace is read and ignored)
///             ^
/// Step 2: "   -42" ('-' is read, so the result should be negative)
///              ^
/// Step 3: "   -42" ("42" is read in)
///                ^
/// The parsed integer is -42.
/// Since -42 is in the range [-231, 231 - 1], the final result is -42.
///
/// Example 3:
/// Input: s = "4193 with words"
/// Output: 4193
/// Explanation:
/// Step 1: "4193 with words" (no characters read because there is no leading whitespace)
///          ^
/// Step 2: "4193 with words" (no characters read because there is neither a '-' nor '+')
///          ^
/// Step 3: "4193 with words" ("4193" is read in; reading stops because the next character is a non-digit)
///              ^
/// The parsed integer is 4193.
/// Since 4193 is in the range [-231, 231 - 1], the final result is 4193.
///

const MINUS_SIGN: char = '-';
const PLUS_SIGN: char = '+';
const WHITESPEACE: char = ' ';

pub fn atoi(s: String) -> i32 {
    use std::collections::HashMap;
    use std::ops::Neg;

    let mut result = 0i32;
    let mut digit_started = false;
    let mut sign: Option<char> = Option::None;

    let digits: HashMap<char, i32> = HashMap::from([
        ('0', 0),
        ('1', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
    ]);

    for c in s.chars() {
        match digits.get(&c) {
            Some(digit) => {
                match result.checked_mul(10) {
                    None => {
                        return match sign {
                            Some(MINUS_SIGN) => { i32::MIN }
                            _ => { i32::MAX }
                        };
                    }
                    Some(scaled) => {
                        match scaled.checked_add(*digit) {
                            Some(new_result) => {
                                result = new_result;
                                digit_started = true;
                            }
                            None => {
                                return match sign {
                                    Some(MINUS_SIGN) => { i32::MIN }
                                    _ => { i32::MAX }
                                };
                            }
                        }
                    }
                }
            }
            None => {
                if digit_started { break; }
                if sign.is_some() { return 0; }
                if c == WHITESPEACE { continue; }
                if c == PLUS_SIGN || c == MINUS_SIGN {
                    if sign.is_some() {
                        return 0;
                    }
                    sign = Some(c);
                    continue;
                }
                return 0;
            }
        }
    }

    match sign {
        Some(MINUS_SIGN) => { result.neg() }
        _ => result
    }
}

#[cfg(test)]
mod tests {
    use super::atoi;
    use rstest::rstest;

    #[rstest(input, expected,
    case("42", 42),
    case("        -42", - 42),
    case(" 4193 with words", 4193),
    case("4193-----", 4193),
    case("words -4193 with words", 0),
    case("0032", 32),
    case("words and 987", 0),
    case("-91283472332", - 2147483648),
    case("+1", 1),
    case("+-12", 0),
    case("2147483648", 2147483647),
    case("-2147483648", - 2147483648),
    case("  +  413", 0),
    )]
    fn test_atoi(input: String, expected: i32) {
        assert_eq!(expected, atoi(input));
    }
}