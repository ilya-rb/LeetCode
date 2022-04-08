/// Two Sum - Arrays::Easy
///
/// Given an array of integers nums and an integer target,
/// return indices of the two numbers such that they add up to target.
///
/// You may assume that each input would have exactly one solution,
/// and you may not use the same element twice.
///
/// You can return the answer in any order.

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut stash: HashMap<i32, usize> = HashMap::new();

    for (left, &item) in nums.iter().enumerate() {
        let remaining = target - item;

        match stash.get(&remaining) {
            None => {
                stash.insert(item, left);
            }
            Some(right) => {
                return vec![left as i32, *right as i32];
            }
        }
    }

    panic!("Sum not found");
}

#[cfg(test)]
mod tests {
    use super::two_sum;
    use rstest::rstest;

    #[rstest(nums, target, expected,
    case(vec ! [0, 1, 2, 3, 4, 7, 11, 15], 6, vec ! [4, 2]),
    case(vec ! [3, - 5, 8, 44, 100, 4, 0], 11, vec ! [2, 0]),
    case(vec ! [3, 2, 4], 6, vec ! [2, 1]),
    )]
    fn test_two_sum(nums: Vec<i32>, target: i32, expected: Vec<i32>) {
        let actual = two_sum(nums, target);
        assert_eq!(expected, actual);
    }
}