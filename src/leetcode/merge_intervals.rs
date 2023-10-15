pub struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut merged_intervals: Vec<Vec<i32>> = vec![];

        for idx in (0..intervals.len()).step_by(2) {
            let current_interval = &intervals[idx];

            if (idx + 1) < intervals.len() {
                let next_interval = &intervals[idx + 1];

                if let Some(merged_interval) =
                    Solution::is_overlaping(current_interval, next_interval)
                {
                    merged_intervals.push(merged_interval);
                } else {
                    merged_intervals.push(current_interval.to_vec());
                    merged_intervals.push(next_interval.to_vec());
                }
            } else {
                merged_intervals.push(current_interval.to_vec());
            }
        }

        return merged_intervals;
    }

    fn is_overlaping(interval_one: &Vec<i32>, interval_two: &Vec<i32>) -> Option<Vec<i32>> {
        if interval_one[0] <= interval_two[1] && interval_two[0] <= interval_one[1] {
            let mut merged_interval = vec![0, 0];

            if interval_one[0] < interval_two[0] {
                merged_interval[0] = interval_one[0];
            } else {
                merged_interval[0] = interval_two[0];
            }

            if interval_one[1] > interval_two[1] {
                merged_interval[1] = interval_one[1];
            } else {
                merged_interval[1] = interval_two[1];
            }

            Some(merged_interval)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];

        assert_eq!(
            Solution::merge(intervals),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
    }
}
