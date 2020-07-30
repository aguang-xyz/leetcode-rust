pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        // If there is only one row, the result is as same as input.
        if num_rows == 1 {
            return s;
        }

        // (x, y) indicates the current position; and d indicates the current
        // direction (0: down, 1: up-right).
        let (mut x, mut y, mut d) = (-1, 0, 0);

        // Retrive all characters with positions.
        let mut vals: Vec<(i32, i32, char)> = s
            .chars()
            .map(|c| {
                if d == 0 {
                    if x + 1 < num_rows {
                        x = x + 1;
                    } else {
                        x = x - 1;
                        y = y + 1;
                        d = 1;
                    }
                } else {
                    if x > 0 {
                        x = x - 1;
                        y = y + 1;
                    } else {
                        x = x + 1;
                        d = 0;
                    }
                }

                (x, y, c)
            })
            .collect();

        // Sort by positions (top first, left first).
        vals.sort_by(|x, y| {
            if x.0 < y.0 {
                Ordering::Less
            } else if x.0 > y.0 {
                Ordering::Greater
            } else if x.1 < y.1 {
                Ordering::Less
            } else if x.1 > y.1 {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        // Convert to final output string.
        vals.into_iter().map(|x| x.2).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0006() {
        assert_eq!(
            Solution::convert(String::from("PAYPALISHIRING"), 3),
            String::from("PAHNAPLSIIGYIR")
        );

        assert_eq!(
            Solution::convert(String::from("PAYPALISHIRING"), 4),
            String::from("PINALSIGYAHRPI")
        );

        assert_eq!(Solution::convert(String::from("AB"), 1), String::from("AB"));
    }
}
