pub struct Solution;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val).reverse()
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.val.cmp(&other.val).reverse())
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let nodes: Vec<ListNode> = lists
            .into_iter()
            .filter(|x| x.is_some())
            .map(|x| *x.unwrap())
            .collect();

        let mut heap: BinaryHeap<ListNode> = BinaryHeap::from(nodes);

        let mut vals: Vec<i32> = vec![];

        while !heap.is_empty() {
            let node: ListNode = heap.pop().unwrap();

            vals.push(node.val);

            if node.next.is_some() {
                heap.push(*node.next.unwrap())
            }
        }

        let mut list: Option<Box<ListNode>> = None;

        for val in vals.into_iter().rev() {
            list = Some(Box::new(ListNode {
                val: val,
                next: list,
            }))
        }

        list
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
    fn test_0023() {
        assert_eq!(
            list2vec(Solution::merge_k_lists(vec![
                vec2list(vec![1, 4, 5]),
                vec2list(vec![1, 3, 4]),
                vec2list(vec![2, 6]),
            ])),
            vec![1, 1, 2, 3, 4, 4, 5, 6]
        );
    }
}
