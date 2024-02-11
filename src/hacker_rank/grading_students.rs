use std::cmp::Ordering::*;

pub struct Solution;

impl Solution {
    pub fn grading_students(grades: &[i32]) -> Vec<i32> {
        let mut rounded_grade = Vec::with_capacity(grades.len());

        for grade in grades {
            match grade.cmp(&38) {
                Less => rounded_grade.push(*grade),
                Equal => rounded_grade.push(40),
                Greater => {
                    let next_multiple = Solution::find_next_multiple(grade);

                    if next_multiple - grade < 3 {
                        rounded_grade.push(next_multiple);
                    } else {
                        rounded_grade.push(*grade);
                    }
                }
            }
        }

        rounded_grade
    }

    fn find_next_multiple(grade: &i32) -> i32 {
        let mut next_multiple = *grade;

        while next_multiple % 5 != 0 {
            next_multiple += 1;
        }

        next_multiple
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grading_students() {
        assert_eq!(
            Solution::grading_students(&[73, 67, 38, 33]),
            vec![75, 67, 40, 33]
        )
    }
}
