pub struct Solution { }

//---
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut start = 0usize;
        let mut end = height.len()-1;
        let mut result = 0;
        while start < end {
            let area = std::cmp::min(height[start],height[end]) * (end-start) as i32;
            result = std::cmp::max(result,area);
            if height[start]<= height[end] {
                start += 1;
            } else {
                end -= 1;
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
        assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
    }
}
