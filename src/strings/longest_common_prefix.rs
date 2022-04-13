/// Write a function to find the longest common prefix string amongst an array of strings.
/// If there is no common prefix, return an empty string "".
///
/// Example 1:
/// Input: strs = ["flower","flow","flight"]
/// Output: "fl"
///
/// Example 2:
/// Input: strs = ["dog","racecar","car"]
/// Output: ""
///
/// Explanation: There is no common prefix among the input strings.
///

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    use std::collections::HashMap;

    let mut registry: HashMap<String, u32> = HashMap::new();
    let len = strs.len() as u32;

    for s in strs.into_iter() {
        for (index, _) in s.chars().enumerate() {
            let part = String::from(&s[0..=index]);
            let count = registry.get(&part).unwrap_or(&0) + 1;
            registry.insert(part, count);
        }
    }

    let (key, value) = registry
        .into_iter()
        .max_by(|a, b| {
            if a.1 == b.1 {
                a.0.len().cmp(&b.0.len())
            } else {
                a.1.cmp(&b.1)
            }
        })
        .unwrap_or((String::new(), 0));

    if value % len != 0 { "".into() } else { key }
}

#[cfg(test)]
mod tests {
    use super::longest_common_prefix;
    use rstest::rstest;

    #[rstest(input, expected,
    case(vec ! ["flower", "flow", "flight"], "fl"),
    case(vec ! ["dog", "racecar", "car"], ""),
    case(vec ! ["a"], "a"),
    case(vec ! ["reflower", "flow", "flight"], ""),
    )]
    fn test_longest_common_prefix(input: Vec<&str>, expected: String) {
        let strings: Vec<String> = input.into_iter().map(|s| String::from(s)).collect();
        assert_eq!(expected, longest_common_prefix(strings));
    }
}