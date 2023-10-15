pub struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut fizz_buzzes = Vec::with_capacity(n as usize);

        for i in 1..=n {
            if i % 3 == 0 && i % 5 == 0 {
                fizz_buzzes.push("FizzBuzz".to_string());
            } else if i % 3 == 0 {
                fizz_buzzes.push("Fizz".to_string());
            } else if i % 5 == 0 {
                fizz_buzzes.push("Buzz".to_string());
            } else {
                fizz_buzzes.push(i.to_string());
            }
        }

        return fizz_buzzes;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizz_buzz() {
        assert_eq!(
            Solution::fizz_buzz(3),
            vec!["1".to_string(), "2".to_string(), "Fizz".to_string()]
        );
    }
}
