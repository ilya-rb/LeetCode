/// Given an integer n, return a string array answer (1-indexed) where:
///
/// answer[i] == "FizzBuzz" if i is divisible by 3 and 5.
/// answer[i] == "Fizz" if i is divisible by 3.
/// answer[i] == "Buzz" if i is divisible by 5.
/// answer[i] == i (as a string) if none of the above conditions are true.
///
/// Example 1:
/// Input: n = 3
/// Output: ["1","2","Fizz"]
///
/// Example 2:
/// Input: n = 5
/// Output: ["1","2","Fizz","4","Buzz"]
///
/// Example 3:
/// Input: n = 15
/// Output: ["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"]
///
pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut result = Vec::with_capacity(n as usize);

    for i in 1..=n {
        if i % 15 == 0 {
            result.push(String::from("FizzBuzz"));
        } else if i % 5 == 0 {
            result.push(String::from("Buzz"));
        } else if i % 3 == 0 {
            result.push(String::from("Fizz"));
        } else {
            result.push(i.to_string());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::fizz_buzz;
    use rstest::rstest;

    #[rstest(n, expected,
    case(3, vec ! [ "1", "2", "Fizz" ]),
    case(5, vec ! [ "1", "2", "Fizz", "4", "Buzz" ]),
    case(15, vec ! [ "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13", "14", "FizzBuzz" ]),
    )]
    fn test_fizz_buzz(n: i32, expected: Vec<&str>) {
        assert_eq!(expected, fizz_buzz(n));
    }
}