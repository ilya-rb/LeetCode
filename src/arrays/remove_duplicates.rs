/// Remove Duplicates from Sorted Array - Arrays::Easy
///
/// Given a sorted array nums, remove the duplicates in-place such that each element
/// appear only once and return the new length.
///
/// Do not allocate extra space for another array, you must do this by modifying the input array
/// in-place with O(1) extra memory.

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 { return 0; }

    let mut i = 0;

    for j in 1..nums.len() {
        let pair = &nums[j - 1..j + 1];

        if pair[0] != pair[1] {
            i += 1;
            nums[i] = nums[j];
        }
    }

    (i + 1) as i32
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::remove_duplicates;

    #[rstest(nums, expected,
    case(vec ! [1, 1, 2], 2),
    case(vec ! [0, 0, 1, 1, 1, 2, 2, 3, 3, 4], 5),
    )]
    fn test_remove_duplicates(mut nums: Vec<i32>, expected: i32) {
        assert_eq!(remove_duplicates(&mut nums), expected)
    }
}