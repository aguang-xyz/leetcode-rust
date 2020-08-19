pub struct Solution {}

use std::cmp::max;
use std::cmp::min;
use std::collections::VecDeque;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;

        let mut ans: i32 = 0;

        let mut que: VecDeque<Vec<usize>> = VecDeque::new();

        que.push_back(vec![]);

        while !que.is_empty() {
            let top: Vec<usize> = que.pop_front().unwrap();

            if top.len() == n {
                ans += 1;
            } else {
                let choices: Vec<usize> = (0..n)
                    .filter(|pos| {
                        if top.contains(pos) {
                            return false;
                        }

                        for (row, &col) in top.iter().enumerate() {
                            if top.len() - row == max(col, *pos) - min(col, *pos) {
                                return false;
                            }
                        }

                        true
                    })
                    .collect();

                for choice in choices.iter().cloned() {
                    let mut new: Vec<usize> = top.clone();

                    new.push(choice);

                    que.push_back(new);
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0052() {
        assert_eq!(Solution::total_n_queens(4), 2);
    }
}
