use std::{collections::HashMap, str::Split};

pub struct Solution;

impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let words = s.split(' ');

        let word_hashmap = Self::to_word_hashmap(words);

        let mut sorted_words = Vec::with_capacity(word_hashmap.len());

        sorted_words.resize(word_hashmap.len(), "placeholder");

        for (index, word) in word_hashmap {
            sorted_words[(index - 1) as usize] = word;
        }

        sorted_words.join(" ").to_string()
    }

    fn to_word_hashmap(words: Split<char>) -> HashMap<u32, &str> {
        let mut word_hashmap: HashMap<u32, &str> = HashMap::new();

        for word in words {
            let word_index = match word.chars().last() {
                Some(index) => index,
                _ => '0',
            };

            let word_without_index = &word[0..word.len() - 1];

            word_hashmap.insert(
                match word_index.to_digit(10) {
                    Some(index) => index,
                    _ => 0,
                },
                word_without_index,
            );
        }

        word_hashmap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_sentence() {
        assert_eq!(
            Solution::sort_sentence("Myself2 Me1 I4 and3".to_string()),
            "Me Myself and I".to_string()
        );
    }
}
