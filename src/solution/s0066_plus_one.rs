pub struct Solution {}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits: Vec<i32> = digits.iter().rev().cloned().collect();
        digits.push(0);
        digits[0] += 1;

        for i in 0..digits.len() {
            if i + 1 < digits.len() {
                digits[i + 1] += digits[i] / 10;
            }

            digits[i] %= 10;
        }

        let mut h: usize = digits.len() - 1;

        while h > 0 && digits[h] == 0 {
            h -= 1;
        }

        (0..(h + 1)).map(|idx| digits[idx]).rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0066() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
        assert_eq!(Solution::plus_one(vec![0]), vec![1]);
    }
}
