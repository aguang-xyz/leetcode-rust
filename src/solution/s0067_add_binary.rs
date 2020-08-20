pub struct Solution {}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a: Vec<i32> = a
            .chars()
            .rev()
            .map(|c| if c == '0' { 0 } else { 1 })
            .collect();

        let b: Vec<i32> = b
            .chars()
            .rev()
            .map(|c| if c == '0' { 0 } else { 1 })
            .collect();

        let mut add: i32 = 0;
        let mut pos: usize = 0;

        let mut ret: Vec<i32> = vec![];

        while add > 0 || pos < a.len() || pos < b.len() {
            if pos < a.len() {
                add += a[pos];
            }

            if pos < b.len() {
                add += b[pos];
            }

            pos += 1;

            ret.push(add % 2);

            add = add / 2;
        }

        ret.iter()
            .map(|&x| if x == 0 { '0' } else { '1' })
            .rev()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0067() {
        assert_eq!(
            Solution::add_binary("11".to_string(), "1".to_string()),
            "100".to_string()
        );

        assert_eq!(
            Solution::add_binary("1010".to_string(), "1011".to_string()),
            "10101".to_string()
        );
    }
}
