pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let temp = head.as_ref();

        if head.is_some() {
            let mut x: ListNode = (**temp.unwrap()).clone();

            if x.next.is_some() {
                let mut y: ListNode = *x.next.unwrap();

                x.next = Solution::swap_pairs(y.next);
                y.next = Some(Box::new(x));

                return Some(Box::new(y));
            }
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list2vec(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec: Vec<i32> = vec![];
        let mut list: Option<Box<ListNode>> = list;

        while list.is_some() {
            let node: ListNode = *list.unwrap();
            vec.push(node.val);
            list = node.next;
        }

        vec
    }

    fn vec2list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut list: Option<Box<ListNode>> = None;

        for val in vec.into_iter().rev() {
            list = Some(Box::new(ListNode {
                val: val,
                next: list,
            }))
        }

        list
    }

    #[test]
    fn test_0024() {
        assert_eq!(
            list2vec(Solution::swap_pairs(vec2list(vec![1, 2, 3, 4]))),
            vec![2, 1, 4, 3]
        );
    }
}
