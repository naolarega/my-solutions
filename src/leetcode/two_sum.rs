pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut index_one = 0;
        let mut index_two = 0;

        let nums_len = nums.len();

        while index_one < nums_len {
            index_two = 0;

            while index_two < nums_len {
                if (&nums[index_one] + &nums[index_two]) == target && index_one != index_two {
                    return vec![index_one as i32, index_two as i32];
                }

                index_two += 1;
            }

            index_one += 1;
        }

        panic!("No number adds up to target!!!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);

        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);

        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
