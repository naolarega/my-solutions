pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut total_prefix = String::new();
        let mut next_char = 0;

        loop {
            let mut prev_char = None;

            for chars in strs.iter().map(|x| x.as_bytes()) {
                if next_char >= chars.len() {
                    return total_prefix;
                }

                match prev_char {
                    None => prev_char = Some(chars[next_char]),
                    Some(current_char) => {
                        if current_char != chars[next_char] {
                            return total_prefix;
                        }
                    }
                }
            }

            if let Some(common_char) = prev_char {
                total_prefix.push(common_char as char);
            }

            next_char += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight")
            ]),
            String::from("fl")
        );

        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("dog"),
                String::from("racecar"),
                String::from("car")
            ]),
            String::from("")
        );

        assert_eq!(
            Solution::longest_common_prefix(vec![String::from("")]),
            String::from("")
        );

        assert_eq!(
            Solution::longest_common_prefix(vec![String::from("a")]),
            String::from("a")
        );
    }
}
