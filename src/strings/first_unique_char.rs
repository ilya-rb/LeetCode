/// Given a string s, find the first non-repeating character in it and return its index.
/// If it does not exist, return -1.
///
/// Example 1:
/// Input: s = "leetcode"
/// Output: 0
///
/// Example 2:
/// Input: s = "loveleetcode"
/// Output: 2
///
/// Example 3:
/// Input: s = "aabb"
/// Output: -1
///

// Leetcode - runtime 39-43ms (0ms using u8 array instead of hash map)
pub fn first_unique_char(s: String) -> i32 {
    use std::collections::HashMap;

    let mut store: HashMap<char, i32> = HashMap::new();

    for c in s.chars() {
        store.insert(c, store.get(&c).unwrap_or(&0) + 1);
    }

    for (index, c) in s.chars().enumerate() {
        if store[&c] == 1 {
            return index as i32;
        }
    }

    -1
}

// Leetcode - runtime ~15ms
fn first_unique_char_2(s: String) -> i32 {
    use std::collections::HashSet;

    let mut processed: HashSet<char> = HashSet::new();

    for (index, c) in s.chars().enumerate() {
        if let None = s[index + 1..].chars().position(|i| i == c) {
            if processed.get(&c).is_none() {
                return index as i32;
            }
        }
        processed.insert(c);
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::first_unique_char;
    use super::first_unique_char_2;
    use rstest::rstest;

    #[rstest(input, expected,
    case("leetcode", 0),
    case("loveleetcode", 2),
    case("aabb", - 1),
    )]
    fn test_first_unique_char(input: &str, expected: i32) {
        assert_eq!(expected, first_unique_char(String::from(input)));
    }

    #[rstest(input, expected,
    case("leetcode", 0),
    case("loveleetcode", 2),
    case("aabb", - 1),
    )]
    fn test_first_unique_char_2(input: &str, expected: i32) {
        assert_eq!(expected, first_unique_char_2(String::from(input)));
    }
}