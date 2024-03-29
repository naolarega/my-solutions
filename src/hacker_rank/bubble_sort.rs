use std::{io, io::BufRead};

pub struct Solution;

impl Solution {
    pub fn bubble_sort() {
        let stdin = io::stdin();
        let mut stdin_iterator = stdin.lock().lines();

        _ = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();

        let mut a: Vec<i32> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();

        Self::count_swaps(&mut a);
    }

    fn count_swaps(input: &mut [i32]) {
        let mut total_swap = 0;

        for _ in 0..input.len() {
            for j in 0..input.len() - 1 {
                if input[j] > input[j + 1] {
                    total_swap += 1;
                    input.swap(j, j + 1);
                }
            }
        }

        println!("Array is sorted in {total_swap} swaps.");
        println!("First Element: {}", input[0]);
        println!("Last Element: {}", input[input.len() - 1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        Solution::count_swaps(&mut [6, 4, 1]);
    }
}
