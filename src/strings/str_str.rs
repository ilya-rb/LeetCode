/// Given two strings needle and haystack,
/// return the index of the first occurrence of needle in haystack, or -1
/// if needle is not part of haystack.
///
/// Clarification:
/// What should we return when needle is an empty string?
/// This is a great question to ask during an interview.
/// For the purpose of this problem, we will return 0 when needle is an empty string.
/// This is consistent to C's strstr() and Java's indexOf().
///
/// Example 1:
/// Input: haystack = "hello", needle = "ll"
/// Output: 2
///
/// Example 2:
/// Input: haystack = "aaaaa", needle = "bba"
/// Output: -1
///

pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.is_empty() || haystack == needle { return 0; }

    let haystack_chars: Vec<char> = haystack.chars().collect();
    let needle_chars: Vec<char> = needle.chars().collect();

    let mut needle_hit_count = 0;
    let mut needle_start_index = Option::None;
    let mut index = 0;

    loop {
        if index == haystack.len() { break; }

        let haystack_char = haystack_chars[index];
        let needle_char = needle_chars[needle_hit_count];

        if haystack_char == needle_char {
            needle_hit_count += 1;

            if needle_start_index.is_none() { needle_start_index = Some(index); }
            if needle_hit_count == needle.len() { return needle_start_index.unwrap() as i32; }

            index += 1;
        } else {
            if needle_start_index.is_some() {
                index = needle_start_index.unwrap() + 1 as usize;
            } else {
                index += 1;
            }
            needle_hit_count = 0;
            needle_start_index = Option::None;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::str_str;
    use rstest::rstest;

    #[rstest(haystack, needle, expected,
    case("hello", "ll", 2),
    case("aaaaa", "bba", - 1),
    case("aaaaa", "", 0),
    case("alohallhellool", "ll", 5),
    case("234tgtdim", "ti", - 1),
    case("aaaaa", "aaaaa", 0),
    case("aaaaa", "a", 0),
    case("aaa", "aa", 0),
    case("mississippi", "issip", 4),
    case("abc", "c", 2),
    case("mississippi", "pi", 9),
    )]
    fn test_str_str(haystack: &str, needle: &str, expected: i32) {
        assert_eq!(expected, str_str(String::from(haystack), String::from(needle)));
    }
}