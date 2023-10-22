pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x > 0 && x < 10 {
            return true
        }

        let x_chars = x.to_string()
            .chars()
            .collect::<Vec<char>>();
        let x_len = x_chars.len();

        for i in 0..x_len / 2 {
            if x_chars[i] != x_chars[(x_len - 1) - i] {
                return false
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome() {
        assert!(Solution::is_palindrome(121));
        assert!(!Solution::is_palindrome(-121));
        assert!(!Solution::is_palindrome(10));
    }
}