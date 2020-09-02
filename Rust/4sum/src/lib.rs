pub struct Solution {}

//---
use std::collections::HashMap;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let nums = nums.as_mut_slice();
        if nums.len() < 4 {
            return vec![];
        }
        nums.sort_unstable();
        let cache = Self::two_sum_cache(nums);
        let mut result = Vec::new();
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let key = target - nums[i] - nums[j];
                if let Some(vec) = cache.get(&key) {
                    for &(m, n) in vec {
                        if i > n {
                            result.push(vec![nums[m], nums[n], nums[i], nums[j]]);
                        }
                    }
                }
            }
        }
        result.sort_unstable();
        result.dedup();
        result
    }

    #[inline(always)]
    fn two_sum_cache(nums: &[i32]) -> HashMap<i32, Vec<(usize, usize)>> {
        let mut cache = HashMap::new();
        for i in 0..nums.len() - 1 {
            for j in i + 1..nums.len() {
                let key = nums[i] + nums[j];
                cache.entry(key).or_insert_with(Vec::new).push((i, j));
            }
        }
        cache
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
