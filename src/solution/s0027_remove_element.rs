pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        for i in 0..(*nums).len() {
            while i < (*nums).len() && (*nums)[i] == val {
                (*nums).remove(i);
            }
        }

        (*nums).len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_0027() {
        let mut vec = vec![3, 2, 2, 3];

        Solution::remove_element(&mut vec, 3);

        assert_eq!(vec, vec![2, 2]);
    }
}
