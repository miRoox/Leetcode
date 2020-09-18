pub struct Solution {}

//---
use std::cmp::min;
use std::cmp::Ordering::*;

impl Solution {
    fn find_kth_slices(s1: &[i32], s2: &[i32], k: usize) -> i32 {
        let n1 = s1.len();
        let n2 = s2.len();
        if n1 > n2 {
            Self::find_kth_slices(s2, s1, k)
        } else if n1 == 0 {
            s2[k - 1]
        } else if k == 1 {
            min(s1[0], s2[0])
        } else {
            let s1i = min(k / 2, n1);
            let s2i = k - s1i;
            match s1[s1i - 1].cmp(&s2[s2i - 1]) {
                Less => Self::find_kth_slices(&s1[s1i..], &s2[..], k - s1i),
                Greater => Self::find_kth_slices(&s1[..], &s2[s2i..], k - s2i),
                Equal => s1[s1i - 1],
            }
        }
    }
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let n1 = nums1.len();
        let n2 = nums2.len();
        let n = n1 + n2;
        if n % 2 == 1 {
            Self::find_kth_slices(&nums1[..], &nums2[..], n / 2 + 1) as f64
        } else {
            (Self::find_kth_slices(&nums1[..], &nums2[..], n / 2)
                + Self::find_kth_slices(&nums1[..], &nums2[..], n / 2 + 1)) as f64
                / 2.0
        }
    }
}
//---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic1() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.0);
    }

    #[test]
    fn basic2() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.5);
    }
}
