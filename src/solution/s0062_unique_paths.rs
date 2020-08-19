pub struct Solution {}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m: usize = m as usize;
        let n: usize = n as usize;

        let mut f: Vec<Vec<i32>> = vec![vec![0; n]; m];

        f[0][0] = 1;

        for i in 0..m {
            for j in 0..n {
                if i > 0 {
                    f[i][j] += f[i - 1][j];
                }

                if j > 0 {
                    f[i][j] += f[i][j - 1];
                }
            }
        }

        f[m - 1][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0062() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
        assert_eq!(Solution::unique_paths(7, 3), 28);
    }
}
