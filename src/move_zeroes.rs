/// Move Zeroes - Arrays::Easy
/// Given an array nums, write a function to move all 0's to the end of it
/// while maintaining the relative order of the non-zero elements.
///
/// Example:
/// Input: [0,1,0,3,12]
/// Output: [1,3,12,0,0]
/// Note:
///
///
/// You must do this in-place without making a copy of the array.
/// Minimize the total number of operations.

/// Time - O(n)
/// Space - O(1)
pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut zeroes_count = 0;

    for i in 0..nums.len() {
        let next = nums[i];
        if next == 0 {
            zeroes_count = zeroes_count + 1;
        } else if zeroes_count > 0 {
            nums[i - zeroes_count] = nums[i];
            nums[i] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::move_zeroes::move_zeroes;
    use rstest::rstest;

    #[rstest(actual, expected,
    case(vec ! [0, 1, 0, 3, 12], vec ! [1, 3, 12, 0, 0]),
    case(vec ! [1, 0, 0, 0, 0], vec ! [1, 0, 0, 0, 0]),
    case(vec ! [2, 1], vec ! [2, 1]),
    )]
    fn test_move_zeroes(expected: Vec<i32>, mut actual: Vec<i32>) {
        move_zeroes(&mut actual);
        assert_eq!(expected, actual);
    }
}