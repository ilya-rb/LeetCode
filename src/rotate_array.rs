/// Rotate Array - Arrays::Easy
/// Given an array, rotate the array to the right by k steps, where k is non-negative.

pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let size = nums.len();
    let k = k as usize;

    if size <= 1 || k == size || k % size == 0 {
        return;
    }

    let steps_count = if k > size {
        k % size
    } else {
        k
    };

    nums.reverse();

    nums[..steps_count].reverse();
    nums[steps_count..].reverse();
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::rotate_array::rotate;

    #[rstest(nums, k, expected,
    case(vec ! [1, 2, 3, 4, 5, 6, 7], 3, vec ! [5, 6, 7, 1, 2, 3, 4]),
    case(vec ! [- 1, - 100, 3, 99], 2, vec ! [3, 99, - 1, - 100]),
    )]
    fn test_rotate(mut nums: Vec<i32>, k: i32, expected: Vec<i32>) {
        rotate(&mut nums, k);
        assert_eq!(nums, expected)
    }
}