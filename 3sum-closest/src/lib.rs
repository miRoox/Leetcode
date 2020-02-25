pub struct Solution {}

//---
use std::cmp::Ordering::*;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let mut result = 0;
        let mut min_diff = std::i32::MAX;
        nums.sort_unstable();
        let last = nums.len() - 1;
        for i1 in 0..last {
            let mut i2 = i1 + 1;
            let mut i3 = last;
            while i2 < i3 {
                let sum = nums[i1] + nums[i2] + nums[i3];
                match sum.cmp(&target) {
                    Less => i2 += 1,
                    Greater => i3 -= 1,
                    Equal => return target,
                }
                let diff = (sum - target).abs();
                if diff < min_diff {
                    result = sum;
                    min_diff = diff;
                }
            }
        }
        result
    }
}
//---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![-1, 2, 1, -4];
        assert_eq!(Solution::three_sum_closest(nums, 1), 2);
    }
}
