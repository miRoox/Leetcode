pub struct Solution {}

//---
use std::cmp::Ordering::*;
macro_rules! unwrap_or_break {
    ($e:expr) => {
        if let Some(&n) = $e {
            n
        } else {
            break;
        }
    };
}

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        const TARGET: i32 = 0;
        let mut result = Vec::new();
        if nums.len() < 3 {
            return result;
        }
        nums.sort_unstable();
        let mut i1 = nums.iter();
        while let Some(&n1) = i1.next() {
            let mut i2 = i1.clone();
            let mut n2 = unwrap_or_break!(i2.next());
            let mut n3 = unwrap_or_break!(i2.next_back());
            loop {
                match (n1 + n2 + n3).cmp(&TARGET) {
                    Less => {
                        n2 = unwrap_or_break!(i2.next());
                    }
                    Greater => {
                        n3 = unwrap_or_break!(i2.next_back());
                    }
                    Equal => {
                        result.push(vec![n1, n2, n3]);
                        n2 = unwrap_or_break!(i2.next());
                        n3 = unwrap_or_break!(i2.next_back());
                    }
                }
            }
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
}
