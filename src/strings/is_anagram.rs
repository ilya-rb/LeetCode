///
/// Given two strings s and t, return true if t is an anagram of s, and false otherwise.
///
/// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase,
/// typically using all the original letters exactly once.

/// Example 1:
/// Input: s = "anagram", t = "nagaram"
/// Output: true
///
/// Example 2:
/// Input: s = "rat", t = "car"
/// Output: false

// Leetcode - Runtime - 0ms
pub fn is_anagram_sort(s: String, t: String) -> bool {
    let mut s_chars: Vec<char> = s.chars().collect();
    s_chars.sort_unstable();

    let mut t_chars: Vec<char> = t.chars().collect();
    t_chars.sort_unstable();

    s_chars == t_chars
}

// Leetcode - Runtime - 8ms
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() { return false; }

    use std::collections::HashMap;

    let mut s_chars = HashMap::new();
    let mut t_chars = HashMap::new();

    for c in s.chars() {
        *s_chars.entry(c).or_insert(1) += 1;
    }

    for c in t.chars() {
        *t_chars.entry(c).or_insert(1) += 1;
    }

    s_chars == t_chars
}

#[cfg(test)]
mod tests {
    use super::is_anagram;
    use super::is_anagram_sort;
    use rstest::rstest;

    #[rstest(first, second, expected,
    case("anagram", "nagaram", true),
    case("rat", "car", false),
    )]
    fn test_is_anagram(first: String, second: String, expected: bool) {
        assert_eq!(expected, is_anagram(first, second));
    }

    #[rstest(first, second, expected,
    case("anagram", "nagaram", true),
    case("rat", "car", false),
    )]
    fn test_is_anagram_sort(first: String, second: String, expected: bool) {
        assert_eq!(expected, is_anagram_sort(first, second));
    }
}