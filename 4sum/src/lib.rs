pub struct Solution {}

//---
use std::cmp::Ordering::*;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let nums = nums.as_mut_slice();
        if nums.len() < 4 {
            return vec![];
        }
        nums.sort_unstable();
        unsafe { Self::four_sum_impl(nums, target) }
    }

    #[inline(always)]
    unsafe fn four_sum_impl(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let start = nums.as_ptr();
        let end = start.add(nums.len() - 1);
        let mut i1 = start;
        while i1 < end.sub(2) {
            let mut i2 = i1.add(1);
            while i2 < end.sub(1) {
                let s2 = *i1 + *i2;
                let mut i3 = i2.add(1);
                let mut i4 = end;
                while i3 < i4 {
                    match (s2 + *i3 + *i4).cmp(&target) {
                        Less => i3 = i3.add(1),
                        Greater => i4 = i4.sub(1),
                        Equal => {
                            result.push(vec![*i1, *i2, *i3, *i4]);
                            i3 = i3.add(1);
                            i4 = i4.sub(1);
                        }
                    }
                }
                i2 = i2.add(1);
            }
            i1 = i1.add(1);
        }
        result.sort_unstable();
        result.dedup();
        result
    }
}
//---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1, 0, -1, 0, -2, 2];
        let result = vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]];
        assert_eq!(Solution::four_sum(nums, 0), result);
    }

    #[test]
    fn zeros() {
        let nums = vec![0, 0, 0, 0];
        let result = vec![vec![0, 0, 0, 0]];
        assert_eq!(Solution::four_sum(nums, 0), result);
    }

    #[test]
    fn negative_target() {
        let nums = vec![1, -2, -5, -4, -3, 3, 3, 5];
        let result = vec![vec![-5, -4, -3, 1]];
        assert_eq!(Solution::four_sum(nums, -11), result);
    }

    #[test]
    fn complex() {
        let nums = vec![-5, -4, -3, -2, -1, 0, 0, 1, 2, 3, 4, 5];
        let result = vec![
            vec![-5, -4, 4, 5],
            vec![-5, -3, 3, 5],
            vec![-5, -2, 2, 5],
            vec![-5, -2, 3, 4],
            vec![-5, -1, 1, 5],
            vec![-5, -1, 2, 4],
            vec![-5, 0, 0, 5],
            vec![-5, 0, 1, 4],
            vec![-5, 0, 2, 3],
            vec![-4, -3, 2, 5],
            vec![-4, -3, 3, 4],
            vec![-4, -2, 1, 5],
            vec![-4, -2, 2, 4],
            vec![-4, -1, 0, 5],
            vec![-4, -1, 1, 4],
            vec![-4, -1, 2, 3],
            vec![-4, 0, 0, 4],
            vec![-4, 0, 1, 3],
            vec![-3, -2, 0, 5],
            vec![-3, -2, 1, 4],
            vec![-3, -2, 2, 3],
            vec![-3, -1, 0, 4],
            vec![-3, -1, 1, 3],
            vec![-3, 0, 0, 3],
            vec![-3, 0, 1, 2],
            vec![-2, -1, 0, 3],
            vec![-2, -1, 1, 2],
            vec![-2, 0, 0, 2],
            vec![-1, 0, 0, 1],
        ];
        assert_eq!(Solution::four_sum(nums, 0), result);
    }
}
