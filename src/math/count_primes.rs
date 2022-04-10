/// Given an integer n, return the number of prime numbers that are strictly less than n.
///
/// Example 1:
/// Input: n = 10
/// Output: 4
/// Explanation: There are 4 prime numbers less than 10, they are 2, 3, 5, 7.
///
/// Example 2:
/// Input: n = 0
/// Output: 0
///
/// Example 3:
/// Input: n = 1
/// Output: 0

pub fn count_primes(n: i32) -> i32 {
    if n <= 2 { return 0; }

    let n = n as usize;
    let sqrt_n = (n as f32).sqrt() as usize;
    let mut primes = vec![true; n];
    primes[0] = false;
    primes[1] = false;

    for i in 2..=sqrt_n {
        if !primes[i] { continue; }
        let mut j = i * i;
        while j < n {
            primes[j] = false;
            j += i;
        }
    }

    primes.into_iter().filter(|&v| { v }).count() as i32
}

#[cfg(test)]
mod tests {
    use super::count_primes;
    use rstest::rstest;

    #[rstest(n, expected,
    case(10, 4),
    case(0, 0),
    case(1, 0),
    case(2, 0),
    case(3, 1),
    case(1000, 168),
    )]
    fn test_count_primes(n: i32, expected: i32) {
        assert_eq!(expected, count_primes(n));
    }
}