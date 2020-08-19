pub struct Solution {}

use std::char;

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut chs: Vec<char> = (1..(n + 1))
            .map(|idx| char::from_digit(idx as u32, 10).unwrap())
            .collect();

        for _i in 1..k {
            Self::next_permutation(&mut chs);
        }

        chs.iter().collect()
    }

    pub fn next_permutation(nums: &mut Vec<char>) {
        let len: usize = nums.len();
        let mut i: usize = len - 1;

        while i > 0 && nums[i - 1] >= nums[i] {
            i -= 1;
        }

        if i > 0 {
            let t: char = nums[i - 1];

            for j in (i..len).rev() {
                if nums[j] > t {
                    nums[i - 1] = nums[j];
                    nums[j] = t;
                    break;
                }
            }
        }

        let mut j: usize = i;
        let mut k: usize = len - 1;

        while j < k {
            let t: char = nums[j];
            nums[j] = nums[k];
            nums[k] = t;

            j += 1;
            k -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_060() {
        assert_eq!(Solution::get_permutation(3, 3), String::from("213"));
        assert_eq!(Solution::get_permutation(4, 9), String::from("2314"));
    }
}
