pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums: Vec<i32> = vec![];
        let (mut i, mut j) = (0, 0);

        while i < nums1.len() || j < nums2.len() {
            if i < nums1.len() && (j == nums2.len() || nums1[i] < nums2[j]) {
                nums.push(nums1[i]);
                i += 1;
            } else {
                nums.push(nums2[j]);
                j += 1;
            }
        }

        if nums.len() % 2 == 0 {
            return (nums[nums.len() / 2 - 1] + nums[nums.len() / 2]) as f64 / 2.0;
        } else {
            return nums[nums.len() / 2] as f64;
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0004() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );

        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }
}
