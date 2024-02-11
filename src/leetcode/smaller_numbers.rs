pub struct Solution;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut small_nums_count = Vec::with_capacity(nums.len());
        let nums_slice = nums.as_slice();

        for idx in 0..nums_slice.len() {
            small_nums_count.push(Solution::count_smaller_nums(nums_slice, idx));
        }

        small_nums_count
    }

    fn count_smaller_nums(nums: &[i32], idx: usize) -> i32 {
        let current_num = nums[idx];
        let mut count = 0;

        for (current_idx, item) in nums.iter().enumerate() {
            if current_idx != idx && *item < current_num {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smaller_numbers_than_current() {
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
            vec![4, 0, 1, 1, 3]
        );
    }
}
