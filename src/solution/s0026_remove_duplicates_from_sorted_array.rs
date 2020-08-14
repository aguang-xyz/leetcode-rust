pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        for i in 1..(*nums).len() {
            while i < (*nums).len() && (*nums)[i - 1] == (*nums)[i] {
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
    pub fn test_0026() {
        let mut vec = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut vec), 2);
        assert_eq!(vec, vec![1, 2]);
    }
}
