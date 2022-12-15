use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

pub struct Solution;

impl Solution {
    pub fn counting_sort() {
        let stdin = io::stdin();
        let mut stdin_iterator = stdin.lock().lines();

        let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();

        let result = Self::sorter(&arr);

        for i in 0..result.len() {
            write!(&mut fptr, "{}", result[i]).ok();

            if i != result.len() - 1 {
                write!(&mut fptr, " ").ok();
            }
        }

        writeln!(&mut fptr).ok();
    }

    fn sorter(arr: &[i32]) -> Vec<i32> {
        let mut frequency_arr = vec![0i32; 100];

        for elem in arr {
            let elem_frequency = frequency_arr[*elem as usize];

            frequency_arr[*elem as usize] = elem_frequency + 1;
        }

        return frequency_arr;
    }
}