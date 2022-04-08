/// Given two integer arrays nums1 and nums2, return an array of their intersection.
/// Each element in the result must appear as many times
/// as it shows in both arrays and you may return the result in any order.
///
/// Example 1:
/// Input: nums1 = [1,2,2,1], nums2 = [2,2]
/// Output: [2,2]
///
/// Example 2:
/// Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
/// Output: [4,9]
/// Explanation: [9,4] is also accepted.

use std::cmp::min;
use std::collections::HashMap;

pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut first_count: HashMap<i32, i32> = HashMap::new();
    let mut second_count: HashMap<i32, i32> = HashMap::new();
    let mut result: Vec<i32> = Vec::new();

    for i in nums1.iter() {
        let current = *first_count.get(&i).unwrap_or(&0);
        first_count.insert(*i, current + 1);
    }

    for i in nums2.iter() {
        let current = *second_count.get(&i).unwrap_or(&0);
        second_count.insert(*i, current + 1);
    }

    for (k, v) in first_count.iter() {
        if let Some(counter_part) = second_count.get(k) {
            let times = min(v, counter_part);
            for _ in 0..*times {
                result.push(*k);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::intersect;
    use rstest::rstest;

    #[rstest(nums1, nums2, expected,
    case(vec ! [1, 2, 2, 1], vec ! [2, 2], vec ! [2, 2]),
    case(vec ! [4, 4, 9, 5], vec ! [9, 4, 9, 8, 4], vec ! [4, 4, 9]),
    case(vec ! [1, 2, 2, 1], vec ! [2], vec ! [2]),
    )]
    fn test_intersect(nums1: Vec<i32>, nums2: Vec<i32>, expected: Vec<i32>) {
        assert_eq!(expected, intersect(nums1, nums2));
    }
}