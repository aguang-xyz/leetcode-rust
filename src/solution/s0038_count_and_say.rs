pub struct Solution {}

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            String::from("1")
        } else {
            let ret = Self::count_and_say(n - 1).chars().fold(
                (String::from(""), 0, '_'),
                |(s, cnt, chr), c| {
                    if c == chr {
                        let cnt = cnt + 1;
                        (s, cnt, chr)
                    } else if cnt > 0 {
                        let s = format!("{}{}{}", s, cnt, chr);
                        (s, 1, c)
                    } else {
                        (s, 1, c)
                    }
                },
            );

            format!("{}{}{}", ret.0, ret.1, ret.2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0038() {
        assert_eq!(Solution::count_and_say(1), String::from("1"));
        assert_eq!(Solution::count_and_say(4), String::from("1211"));
    }
}
