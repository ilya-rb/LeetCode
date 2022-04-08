/// Contains Duplicate - Arrays::Easy
/// Given an array of integers, find if the array contains any duplicates.
///
/// Your function should return true if any value appears at least twice in the array,
/// and it should return false if every element is distinct.

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    use std::collections::HashSet;

    let mut registry: HashSet<i32> = HashSet::new();

    for i in nums.iter() {
        if registry.contains(i) {
            return true;
        }
        registry.insert(*i);
    }

    false
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::contains_duplicate;

    #[rstest(input, expected,
    case(vec ! [1, 2, 3, 1], true),
    case(vec ! [1, 2, 3, 4], false),
    case(vec ! [1, 1, 1, 3, 3, 4, 3, 2, 4, 2], true),
    )]
    fn test_contains_duplicate(input: Vec<i32>, expected: bool) {
        assert_eq!(contains_duplicate(input), expected);
    }
}