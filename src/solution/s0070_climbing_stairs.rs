pub struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;
        let mut f: Vec<i32> = vec![0; n + 1];

        f[0] = 1;

        for i in 1..(n + 1) {
            if i >= 1 {
                f[i] += f[i - 1];
            }

            if i >= 2 {
                f[i] += f[i - 2];
            }
        }

        f[n]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0070() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}
