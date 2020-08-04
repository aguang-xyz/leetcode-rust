pub struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        // Map nums from [-1000, 1000] to [0, 2000].
        let nums: Vec<i32> = nums.into_iter().map(|num| num + 1000).collect();
        let target = target + 3000;

        // f[i][j][k] indicates whether we have a solution selecting j numbers
        // from the first i elements that the sum is k.
        let mut f: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 6001]; 4]; nums.len() + 1];

        f[0][0][0] = true;

        for i in 0..(nums.len() + 1) {
            for j in 0..4 {
                for k in 0..6001 {
                    if i == 0 && j == 0 && k == 0 {
                        f[i][j][k] = true
                    } else {
                        f[i][j][k] = false;

                        if i > 0 && f[i - 1][j][k] {
                            f[i][j][k] = true
                        } else if i > 0
                            && j > 0
                            && k as i32 - nums[i - 1] >= 0
                            && k as i32 - nums[i - 1] <= 6000
                            && f[i - 1][j - 1][k - nums[i - 1] as usize]
                        {
                            f[i][j][k] = true
                        }
                    }
                }
            }
        }

        let mut ans: i32 = -1;
        let mut num: i32 = -1;

        for k in 0..6000 {
            if f[nums.len()][3][k] {
                if num == -1 || (k as i32 - target).abs() < ans {
                    ans = (k as i32 - target).abs();
                    num = k as i32;
                }
            }
        }

        num - 3000
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0016() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    }
}
