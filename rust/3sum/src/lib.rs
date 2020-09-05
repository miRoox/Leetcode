pub struct Solution {}

//---
use std::cmp::Ordering::*;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let nums = nums.as_mut_slice();
        if nums.len() < 3 {
            return vec![];
        }
        nums.sort_unstable();
        unsafe { Self::three_sum_impl(nums) }
    }

    #[inline(always)]
    unsafe fn three_sum_impl(nums: &[i32]) -> Vec<Vec<i32>> {
        const TARGET: i32 = 0;
        let mut result = Vec::new();
        let start = nums.as_ptr();
        let end = start.add(nums.len() - 1);
        let mut i1 = start;
        while i1 < end {
            if *i1 > TARGET {
                break;
            }
            while i1 > start && *i1 == *i1.sub(1) {
                i1 = i1.add(1);
            }
            let mut i2 = i1.add(1);
            let mut i3 = end;
            while i2 < i3 {
                match (*i1 + *i2 + *i3).cmp(&TARGET) {
                    Less => {
                        i2 = i2.add(1);
                    }
                    Greater => {
                        i3 = i3.sub(1);
                    }
                    Equal => {
                        result.push(vec![*i1, *i2, *i3]);
                        while i2 < i3 && *i2 == *i2.add(1) {
                            i2 = i2.add(1);
                        }
                        while i2 < i3 && *i3 == *i3.sub(1) {
                            i3 = i3.sub(1);
                        }
                        i2 = i2.add(1);
                        i3 = i3.sub(1);
                    }
                }
            }
            i1 = i1.add(1);
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
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let result = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(Solution::three_sum(nums), result);
    }

    #[test]
    fn many_dup() {
        let nums = vec![-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6];
        let result = vec![
            vec![-4, -2, 6],
            vec![-4, 0, 4],
            vec![-4, 1, 3],
            vec![-4, 2, 2],
            vec![-2, -2, 4],
            vec![-2, 0, 2],
        ];
        assert_eq!(Solution::three_sum(nums), result);
    }

    #[test]
    fn zeros() {
        let nums = vec![0, 0, 0, 0];
        let result = vec![vec![0, 0, 0]];
        assert_eq!(Solution::three_sum(nums), result);
    }
}
