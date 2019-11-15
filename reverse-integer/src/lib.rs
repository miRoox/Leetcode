pub struct Solution { }

//---
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut ans = 0i32;
        while x!=0 {
            if let Some(try_mul) = ans.checked_mul(10) {
                if let Some(try_add) = try_mul.checked_add(x%10) {
                    ans = try_add; // ans = 10*ans+x%10
                    x /= 10;
                } else {
                    return 0;
                }
            } else {
                return 0;
            }
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

    #[test]
    fn overflow() {
        assert_eq!(Solution::reverse(1534236469), 0);
    }

    #[test]
    fn underflow() {
        assert_eq!(Solution::reverse(-1534236469), 0);
    }
}
