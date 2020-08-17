pub struct Solution {}

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let len: usize = nums.len();
        let mut i: usize = len - 1;

        while i > 0 && nums[i - 1] >= nums[i] {
            i -= 1;
        }

        if i > 0 {
            let t: i32 = nums[i - 1];

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
            let t: i32 = nums[j];
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
    fn test_0031() {
        let mut vec: Vec<i32> = vec![1, 2, 3];
        Solution::next_permutation(&mut vec);
        assert_eq!(vec, vec![1, 3, 2]);

        let mut vec: Vec<i32> = vec![3, 2, 1];
        Solution::next_permutation(&mut vec);
        assert_eq!(vec, vec![1, 2, 3]);

        let mut vec: Vec<i32> = vec![1, 1, 5];
        Solution::next_permutation(&mut vec);
        assert_eq!(vec, vec![1, 5, 1]);

        let mut vec: Vec<i32> = vec![1, 3, 2];
        Solution::next_permutation(&mut vec);
        assert_eq!(vec, vec![2, 1, 3]);
    }
}
