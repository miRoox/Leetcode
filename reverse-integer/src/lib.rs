pub struct Solution { }

//---
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut ans = 0;
        while x!=0 {
            let pop = x%10;
            ans = 10*ans + pop;
            x /= 10;
        }
        ans
    }
}
//---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive() {
        assert_eq!(Solution::reverse(123), 321);
    }

    #[test]
    fn negative() {
        assert_eq!(Solution::reverse(-123), -321);
    }

    #[test]
    fn ends_zero() {
        assert_eq!(Solution::reverse(120), 21);
    }
}
