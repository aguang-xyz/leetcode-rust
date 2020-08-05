pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Solution {
    pub fn len(head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, i32) {
        let mut list = head.as_ref();
        let mut ret = 0;

        while list.is_some() {
            ret = ret + 1;
            list = list.unwrap().next.as_ref();
        }

        (head, ret)
    }

    pub fn removed(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        if n == 0 {
            head.unwrap().next
        } else {
            let val = head.as_ref().unwrap().val;

            Some(Box::new(ListNode {
                next: Solution::removed(head.unwrap().next, n - 1),
                val,
            }))
        }
    }

    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let len = Solution::len(head);

        Solution::removed(len.0, len.1 - n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0019() {
        let mut list: Option<Box<ListNode>> = None;

        for num in vec![1, 2, 3, 4, 5].into_iter().rev() {
            list = Some(Box::new(ListNode {
                next: list,
                val: num,
            }));
        }

        let ret = Solution::remove_nth_from_end(list, 2);
        let mut ret = ret.as_ref();

        let mut nums: Vec<i32> = vec![];

        while ret.is_some() {
            let ptr: &Box<ListNode> = ret.unwrap();
            nums.push(ptr.val);
            ret = ptr.next.as_ref();
        }

        assert_eq!(vec![1, 2, 3, 5], nums);
    }
}
