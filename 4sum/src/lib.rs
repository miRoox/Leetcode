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
        unsafe { Self::four_sum_impl(nums,target) }
    }

    #[inline(always)]
    unsafe fn four_sum_impl(nums : &[i32], target : i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let start = nums.as_ptr();
        let end = start.add(nums.len() - 1);
        let mut i1 = start;
        'main: while i1 < end.sub(2) {
            if *i1 > target {
                break 'main;
            }
            Self::skip_dup(&mut i1, &start);
            let mut i2 = i1.add(1);
            while i2 < end.sub(1) {
                Self::skip_dup(&mut i2, &start);
                let s2 = *i1 + *i2;
                if s2 > target {
                    break 'main;
                }
                let mut i3 = i2.add(1);
                let mut i4 = end;
                while i3 < i4 {
                    match (s2 + *i3 +*i4).cmp(&target) {
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
        result
    }

    #[inline(always)]
    unsafe fn skip_dup(i : &mut *const i32, start : &* const i32) {
        while *i > *start && **i == *i.sub(1) {
            *i = i.add(1);
        }
    }
}
//---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1, 0, -1, 0, -2, 2];
        let result = vec![
            vec![-2, -1, 1, 2],
            vec![-2,  0, 0, 2],
            vec![-1,  0, 0, 1]
        ];
        assert_eq!(Solution::four_sum(nums, 0), result);
    }
}
