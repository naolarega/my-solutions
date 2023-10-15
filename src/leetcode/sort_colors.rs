pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut i = 0;
        let nums_len = nums.len();

        while i < nums_len {
            let mut j = i;

            while j < nums_len {
                if nums[j] < nums[i] {
                    let temp = nums[i];
                    nums[i] = nums[j];
                    nums[j] = temp;
                }
                j += 1;
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_color() {
        let mut unsorted_colors = vec![2, 0, 2, 1, 1, 0];

        Solution::sort_colors(&mut unsorted_colors);

        assert_eq!(unsorted_colors, vec![0, 0, 1, 1, 2, 2]);
    }
}
