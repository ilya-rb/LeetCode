/// Given a signed 32-bit integer x, return x with its digits reversed.
/// If reversing x causes the value to go outside the signed 32-bit integer range [-231, 231 - 1],
/// then return 0.
///
/// Assume the environment does not allow you to store 64-bit integers (signed or unsigned).

pub fn reverse_integer(x: i32) -> i32 {
    let negative = x < 0;

    let mut result = 0i32;
    let mut x = x.abs();

    while x > 0 {
        let next = x % 10;
        let next_scale = result.checked_mul(10);
        if next_scale.is_none() {
            return 0;
        }
        result = next_scale.unwrap() + next;
        x /= 10;
    }

    if negative { -result } else { result }
}

#[cfg(test)]
mod tests {
    use super::reverse_integer;
    use rstest::rstest;

    #[rstest(input, expected,
    case(123, 321),
    case(- 123, - 321),
    case(120, 21),
    case(900000, 9),
    case(1534236469, 0),
    )]
    fn test_reverse_integer(input: i32, expected: i32) {
        assert_eq!(expected, reverse_integer(input));
    }
}

