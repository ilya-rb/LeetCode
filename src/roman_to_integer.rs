/// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
///
/// Symbol       Value
/// I             1
/// V             5
/// X             10
/// L             50
/// C             100
/// D             500
/// M             1000
///
/// For example, 2 is written as II in Roman numeral, just two one's added together.
/// 12 is written as XII, which is simply X + II.
/// The number 27 is written as XXVII, which is XX + V + II.
///
/// Roman numerals are usually written largest to smallest from left to right.
/// However, the numeral for four is not IIII. Instead, the number four is written as IV.
/// Because the one is before the five we subtract it making four.
/// The same principle applies to the number nine, which is written as IX.
/// There are six instances where subtraction is used:
///
/// I can be placed before V (5) and X (10) to make 4 and 9.
/// X can be placed before L (50) and C (100) to make 40 and 90.
/// C can be placed before D (500) and M (1000) to make 400 and 900.
/// Given a roman numeral, convert it to an integer.

pub fn roman_to_integer(s: String) -> i32 {
    use std::collections::HashMap;

    let mut result = 0;
    let numbers: HashMap<char, i32> = HashMap::from(
        [
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]
    );

    for (index, c) in s.chars().enumerate() {
        let next = numbers[&c];

        if index == 0 {
            result += next;
            continue;
        }

        let previous = s.chars().nth(index - 1).unwrap();

        match (&previous, &c) {
            ('I', 'V') | ('I', 'X') | ('X', 'L') | ('X', 'C') | ('C', 'D') | ('C', 'M') => {
                result += next - numbers[&previous] * 2;
            }
            _ => {
                result += next;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::roman_to_integer::roman_to_integer;
    use rstest::rstest;

    #[rstest(input, expected,
    case("III", 3),
    case("LVIII", 58),
    case("MCMXCIV", 1994)
    )]
    fn test_roman_to_integer(input: &str, expected: i32) {
        assert_eq!(expected, roman_to_integer(input.to_string()));
    }
}