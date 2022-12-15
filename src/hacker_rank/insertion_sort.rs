use std::io::{self, BufRead, Write};

pub struct Solution;

impl Solution {
    pub fn insertion_sort() {
        let stdin = io::stdin();
        let mut stdin_iterator = stdin.lock().lines();
    
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    
        let mut arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    
        Self::sorter(n, &mut arr);
    }

    fn sorter(n: i32, arr: &mut [i32]) {
        let last_elem = arr[(n - 1) as usize];

        for idx in (0..arr.len() - 1).rev() {
            if arr[idx] > last_elem {
                arr[idx + 1] = arr[idx];

                Self::print_arr(arr);
            } else {
                arr[idx + 1] = last_elem;
                
                Self::print_arr(arr);
                
                return;
            }
        }

        arr[0] = last_elem;

        Self::print_arr(arr);
    }

    fn print_arr(arr: &[i32]) {
        let arr_len = arr.len();

        for idx in 0..arr_len {
            if idx < arr_len - 1 {
                print!("{} ", arr[idx]);
            } else {
                print!("{}\n", arr[idx])
            }
        }

        io::stdout().flush().unwrap();
    }
}