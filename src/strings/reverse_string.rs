/// Write a function that reverses a string.
/// The input string is given as an array of characters s.
///
/// You must do this by modifying the input array in-place with O(1) extra memory.
///
/// Example 1:
/// Input: s = ["h","e","l","l","o"]
/// Output: ["o","l","l","e","h"]
///
/// Example 2:
/// Input: s = ["H","a","n","n","a","h"]
/// Output: ["h","a","n","n","a","H"]

pub fn reverse_string(s: &mut Vec<char>) {
    let mut i = 0;
    let mut j = s.len() - 1;

    while i < j {
        s.swap(i, j);
        i += 1;
        j -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::reverse_string;
    use rstest::rstest;

    #[rstest(input, expected,
    case(
    & mut vec ! ['h', 'e', 'l', 'l', 'o'],
    & mut vec ! ['o', 'l', 'l', 'e', 'h'],
    ),
    case(
    & mut vec ! ['H', 'a', 'n', 'n', 'a', 'h'],
    & mut vec ! ['h', 'a', 'n', 'n', 'a', 'H'],
    ),
    )]
    fn test_reverse_string(input: &mut Vec<char>, expected: &mut Vec<char>) {
        reverse_string(input);
        assert_eq!(expected, input);
    }
}