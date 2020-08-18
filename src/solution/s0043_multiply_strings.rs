pub struct Solution {}

use std::char;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let num1: Vec<u32> = num1
            .chars()
            .rev()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        let num2: Vec<u32> = num2
            .chars()
            .rev()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        let mut ans: Vec<u32> = vec![0; num1.len() + num2.len()];

        for i in 0..num1.len() {
            for j in 0..num2.len() {
                ans[i + j] += num1[i] * num2[j];
            }
        }

        for i in 0..ans.len() {
            if i + 1 < ans.len() {
                ans[i + 1] += ans[i] / 10;
            }

            ans[i] %= 10;
        }

        let mut idx: usize = ans.len() - 1;

        while ans[idx] == 0 && idx > 0 {
            idx -= 1;
        }

        ans[0..(idx + 1)]
            .iter()
            .rev()
            .cloned()
            .map(|x| char::from_digit(x, 10).unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0043() {
        assert_eq!(
            Solution::multiply(String::from("2"), String::from("3")),
            String::from("6")
        );

        assert_eq!(
            Solution::multiply(String::from("123"), String::from("456")),
            String::from("56088")
        );
    }
}
