pub struct Solution;

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let chs: Vec<char> = str.chars().collect();

        let mut neg: bool = false;
        let mut idx: usize = 0;

        // Skip whitespace characters at the beginning.
        while idx < chs.len() && chs[idx] == ' ' {
            idx += 1;
        }

        // Return 0 if all characters are whitespaces.
        if idx == chs.len() {
            return 0;
        }

        // Parse starting +/-.
        match chs[idx] {
            '-' => {
                neg = true;
                idx += 1;
            }
            '+' => {
                idx += 1;
            }
            _ => (),
        }

        let mut val: i64 = 0;

        // Parse digits.
        while idx < chs.len() && chs[idx] >= '0' && chs[idx] <= '9' {
            val = val * 10 + (chs[idx] as i64 - '0' as i64);
            idx += 1;

            if neg && -val < i32::min_value() as i64 {
                val = -(i32::min_value() as i64)
            }

            if !neg && val > i32::max_value() as i64 {
                val = i32::max_value() as i64
            }
        }

        if neg {
            val = -val;
        }

        val as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0001() {
        assert_eq!(Solution::my_atoi(String::from("42")), 42);
        assert_eq!(Solution::my_atoi(String::from("   -42")), -42);
        assert_eq!(Solution::my_atoi(String::from("4193 with words")), 4193);
        assert_eq!(Solution::my_atoi(String::from("words and 987")), 0);
        assert_eq!(Solution::my_atoi(String::from("-91283472332")), -2147483648);
        assert_eq!(
            Solution::my_atoi(String::from("9223372036854775808")),
            2147483647
        );
    }
}
