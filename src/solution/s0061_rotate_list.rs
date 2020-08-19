pub struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Solution {
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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let k: usize = k as usize;

        let vec: Vec<i32> = Self::list_to_vec(head);

        let vec: Vec<i32> = (0..vec.len())
            .map(|idx| (idx + vec.len() * k - k) % vec.len())
            .map(|idx| vec[idx])
            .collect();

        Self::vec_to_list(vec)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0061() {
        assert_eq!(
            Solution::list_to_vec(Solution::rotate_right(
                Solution::vec_to_list(vec![1, 2, 3, 4, 5]),
                2
            )),
            vec![4, 5, 1, 2, 3]
        );

        assert_eq!(
            Solution::list_to_vec(Solution::rotate_right(
                Solution::vec_to_list(vec![0, 1, 2]),
                4
            )),
            vec![2, 0, 1]
        );
    }
}
