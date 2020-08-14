pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Solution {
    fn join(stack: Option<Box<ListNode>>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if stack.is_some() {
            let node: ListNode = *stack.unwrap();

            Some(Box::new(ListNode {
                val: node.val,
                next: Self::join(node.next, head),
            }))
        } else {
            head
        }
    }

    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let pop_all = |mut stack: Option<Box<ListNode>>, mut head: Option<Box<ListNode>>| {
            while stack.is_some() {
                let node: ListNode = *stack.unwrap();

                head = Some(Box::new(ListNode {
                    val: node.val,
                    next: head,
                }));

                stack = node.next;
            }

            head
        };

        let mut head: Option<Box<ListNode>> = head;
        let mut stack: Option<Box<ListNode>> = None;

        for _in in 0..k {
            if head.is_some() {
                let node: ListNode = *head.unwrap();

                stack = Some(Box::new(ListNode {
                    val: node.val,
                    next: stack,
                }));

                head = node.next;
            } else {
                return pop_all(stack, head);
            }
        }

        Self::join(stack, Self::reverse_k_group(head, k))
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
    pub fn test_0025() {
        assert_eq!(
            list2vec(Solution::reverse_k_group(vec2list(vec![1, 2, 3, 4, 5]), 2)),
            vec![2, 1, 4, 3, 5],
        );

        assert_eq!(
            list2vec(Solution::reverse_k_group(vec2list(vec![1, 2, 3, 4, 5]), 3)),
            vec![3, 2, 1, 4, 5],
        );
    }
}
