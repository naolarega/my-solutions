use std::{io, io::BufRead};

pub struct Solution;

impl Solution {
    pub fn bubble_sort() {
        let stdin = io::stdin();
        let mut stdin_iterator = stdin.lock().lines();

        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let mut a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();

        Self::count_swaps(&mut a);
    }

    fn count_swaps(input: &mut [i32]) {
        let mut total_swap = 0;

        for i in 0..input.len() {
            for j in 0..input.len() - 1 {
                if input[j] > input[j + 1] {
                    total_swap += 1;
                    Self::swap(input, j);
                }
            }
        }

        println!("Array is sorted in {total_swap} swaps.");
        println!("First Element: {}", input[0]);
        println!("Last Element: {}", input[input.len() - 1]);
    }

    fn swap<'a>(elems: &'a mut [i32], index: usize) {
        let temp_elem = elems[index];

        elems[index] = elems[index + 1];
        elems[index + 1] = temp_elem;
    }
}