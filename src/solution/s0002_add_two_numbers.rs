pub struct Solution {}

pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();
        let mut list = list.as_ref();

        while let Some(node) = list {
            ret.push(node.val);

            list = node.next.as_ref();
        }

        ret
    }

    pub fn from_vec(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut ret: Option<Box<ListNode>> = None;

        for num in nums.iter().rev() {
            ret = Some(Box::new(ListNode {
                val: *num,
                next: ret,
            }))
        }

        ret
    }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);

        let mut add = 0;
        let mut vals: Vec<i32> = Vec::new();

        while l1.is_some() || l2.is_some() || add > 0 {
            let mut val = add;

            l1 = match l1 {
                Some(node) => {
                    val += node.val;
                    node.next
                }

                None => None,
            };

            l2 = match l2 {
                Some(node) => {
                    val += node.val;
                    node.next
                }

                None => None,
            };

            add = val / 10;
            val = val % 10;

            vals.push(val);
        }

        let mut ret: Option<Box<ListNode>> = None;

        for val in vals.iter().rev() {
            ret = Some(Box::new(ListNode {
                val: *val,
                next: ret,
            }));
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0002() {
        assert_eq!(
            ListNode::to_vec(Solution::add_two_numbers(
                ListNode::from_vec(vec![2, 4, 3]),
                ListNode::from_vec(vec![5, 6, 4]),
            )),
            vec![7, 0, 8]
        )
    }
}
