#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut results = Vec::new();

        let mut current_l1 = l1;
        let mut current_l2 = l2;

        let mut overflows = false;

        loop {
            let mut sum = 0;

            if overflows {
                sum += 1;
            }

            if let Some(list_node) = current_l1 {
                sum += list_node.val;
                current_l1 = list_node.next;
            }

            if let Some(list_node) = current_l2 {
                sum += list_node.val;
                current_l2 = list_node.next;
            }

            if sum >= 10 {
                overflows = true;
                sum -= 10;
            }

            results.push(sum);

            if current_l1.is_none() && current_l2.is_none() {
                break;
            }
        }

        let mut l_result: Option<Box<ListNode>> = None;

        for val in results.iter().rev() {
            l_result = Some(Box::new(ListNode {
                val: *val,
                next: l_result,
            }));
        }

        l_result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        assert_eq!(
            Solution::add_two_numbers(
                Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 3, next: None }))
                    }))
                })),
                Some(Box::new(ListNode {
                    val: 5,
                    next: Some(Box::new(ListNode {
                        val: 6,
                        next: Some(Box::new(ListNode { val: 4, next: None }))
                    }))
                }))
            ),
            Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode { val: 8, next: None }))
                }))
            }))
        )
    }
}
