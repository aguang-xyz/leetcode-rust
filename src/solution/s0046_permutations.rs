pub struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() <= 1 {
            vec![nums.clone()]
        } else {
            let mut ans: Vec<Vec<i32>> = vec![];

            for num in nums.iter().cloned() {
                let sub_rets: Vec<Vec<i32>> =
                    Self::permute(nums.iter().cloned().filter(|&x| x != num).collect());

                for sub_ret in sub_rets.iter() {
                    let mut new_ret = sub_ret.clone();
                    new_ret.insert(0, num);
                    ans.push(new_ret);
                }
            }

            ans
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0046() {
        assert_eq!(
            Solution::permute(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }
}
