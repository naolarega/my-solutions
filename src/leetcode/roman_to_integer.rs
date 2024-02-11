use std::{cmp::Ordering, collections::HashMap};

pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut roman_buffer: Option<char> = None;
        let mut integer_value = 0;
        let mut temp_intger_value = 0;

        let roman_integer_map = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);

        for i in s.chars() {
            let cur = roman_integer_map.get(&i).unwrap();

            if let Some(roman_buffer) = roman_buffer {
                let prev = roman_integer_map.get(&roman_buffer).unwrap();

                match prev.cmp(cur) {
                    Ordering::Less => temp_intger_value = cur - prev,
                    Ordering::Equal => temp_intger_value += cur,
                    Ordering::Greater => {
                        integer_value += temp_intger_value;
                        temp_intger_value = *cur;
                    }
                }
            } else {
                temp_intger_value += cur;
            }

            roman_buffer = Some(i);
        }

        integer_value + temp_intger_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);

        assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);

        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    }
}
