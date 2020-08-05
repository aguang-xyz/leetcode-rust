pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() {
            return l2;
        }

        if l2.is_none() {
            return l1;
        }

        let l1 = l1.unwrap();
        let l2 = l2.unwrap();

        let v1 = l1.val;
        let v2 = l2.val;

        if v1 < v2 {
            Some(Box::new(ListNode {
                next: Solution::merge_two_lists(l1.next, Some(l2)),
                val: v1,
            }))
        } else {
            Some(Box::new(ListNode {
                next: Solution::merge_two_lists(l2.next, Some(l1)),
                val: v2,
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec_to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut list: Option<Box<ListNode>> = None;

        for num in nums.into_iter().rev() {
            list = Some(Box::new(ListNode {
                next: list,
                val: num,
            }));
        }

        list
    }

    fn list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut nums: Vec<i32> = vec![];
        let mut list = head.as_ref();

        while list.is_some() {
            let ptr = list.unwrap();

            nums.push(ptr.val);

            list = ptr.next.as_ref();
        }

        nums
    }

    #[test]
    fn test_0021() {
        assert_eq!(
            list_to_vec(Solution::merge_two_lists(
                vec_to_list(vec![1, 2, 4]),
                vec_to_list(vec![1, 3, 4])
            )),
            vec![1, 1, 2, 3, 4, 4]
        );
    }
}
