/// A phrase is a palindrome if, after converting all uppercase letters into lowercase letters
/// and removing all non-alphanumeric characters, it reads the same forward and backward.
/// Alphanumeric characters include letters and numbers.
///
/// Given a string s, return true if it is a palindrome, or false otherwise.
///
/// Example 1:
/// Input: s = "A man, a plan, a canal: Panama"
/// Output: true
/// Explanation: "amanaplanacanalpanama" is a palindrome.
///
/// Example 2:
/// Input: s = "race a car"
/// Output: false
/// Explanation: "raceacar" is not a palindrome.
///
/// Example 3:
/// Input: s = " "
/// Output: true
/// Explanation: s is an empty string "" after removing non-alphanumeric characters.
///
/// Since an empty string reads the same forward and backward, it is a palindrome.

pub fn is_palindrome(s: String) -> bool {
    let reversed_iter = s.chars().rev().filter(|c| c.is_alphanumeric());
    let reversed = String::from_iter(reversed_iter).to_lowercase().replace(" ", "");
    let s_iter = s.chars().filter(|c| c.is_alphanumeric());
    let s_normalized = String::from_iter(s_iter).to_lowercase().replace(" ", "");

    reversed == s_normalized
}

#[cfg(test)]
mod tests {
    use super::is_palindrome;
    use rstest::rstest;

    #[rstest(input, expected,
    case("A man, a plan, a canal: Panama", true),
    case("race a car", false),
    case(" ", true),
    case("", true),
    case("0P", false),
    )]
    fn test_is_palindrome(input: String, expected: bool) {
        assert_eq!(expected, is_palindrome(input));
    }
}