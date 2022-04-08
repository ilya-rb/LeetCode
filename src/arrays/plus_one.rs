/// Plus One - Arrays::Easy
///
/// Given a non-empty array of digits representing a non-negative integer,
/// increment one to the integer.
/// The digits are stored such that the most significant digit is at the head of the list,
/// and each element in the array contains a single digit.
/// You may assume the integer does not contain any leading zero, except the number 0 itself.
///
/// Time: O(n) - worst case
/// Space: O(1)
pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let mut index = digits.len();
    let mut need_to_inc_prev = true;

    while need_to_inc_prev && index > 0 {
        digits[index - 1] = if digits[index - 1] == 9 {
            0
        } else {
            digits[index - 1] + 1
        };
        need_to_inc_prev = digits[index - 1] == 0;
        index = index - 1;
    }

    if index == 0 && digits[index] == 0 {
        digits.insert(0, 1);
    }

    digits
}

#[cfg(test)]
mod tests {
    use super::plus_one;
    use rstest::rstest;

    #[rstest(input, expected,
    case(vec ! [1, 2, 3], vec ! [1, 2, 4]),
    case(vec ! [4, 3, 2, 1], vec ! [4, 3, 2, 2]),
    case(vec ! [0], vec ! [1]),
    case(vec ! [9, 9], vec ! [1, 0, 0]),
    case(vec ! [9, 8, 7, 6, 5, 4, 3, 2, 1, 0], vec ! [9, 8, 7, 6, 5, 4, 3, 2, 1, 1])
    )]
    fn plus_one_test(input: Vec<i32>, expected: Vec<i32>) {
        assert_eq!(plus_one(input), expected);
    }
}