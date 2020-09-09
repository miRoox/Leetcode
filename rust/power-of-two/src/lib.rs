pub struct Solution {}

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        // see https://www.bilibili.com/video/BV1rz4y1Q7z8
        n > 0 && ((n & -n) == n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        assert!(Solution::is_power_of_two(1));
    }

    #[test]
    fn even() {
        assert!(!Solution::is_power_of_two(218));
    }

    #[test]
    fn negative() {
        assert!(!Solution::is_power_of_two(-8));
    }
}
