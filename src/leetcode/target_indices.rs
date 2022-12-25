pub struct Solution;

impl Solution {
    pub fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut indices: Vec<i32> = Vec::new();

        nums.sort();
        
        for (idx, num) in nums.iter().enumerate() {
            if num == &target {
                indices.push(idx as i32);
            }
        }

        return indices;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_indices() {
        let indices = Solution::target_indices(vec![1, 2, 5, 2, 3], 2);
        assert_eq!(indices, vec![1, 2]);
    }
}