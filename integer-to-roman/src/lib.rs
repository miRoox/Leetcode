pub struct Solution { }

//---
impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let radix = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let symbol  = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
        let mut roman = String::new();
        let mut i = 0usize;
        while num>0 {
            let time = (num/radix[i]) as usize;
            num %= radix[i];
            roman += symbol[i].repeat(time).as_str();
            i +=1 ;
        }
        roman
    }
}
//---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_3() {
        assert_eq!(Solution::int_to_roman(3), "III".to_string());
    }

    #[test]
    fn for_4() {
        assert_eq!(Solution::int_to_roman(4), "IV".to_string());
    }

    #[test]
    fn for_9() {
        assert_eq!(Solution::int_to_roman(9), "IX".to_string());
    }

    #[test]
    fn for_58() {
        assert_eq!(Solution::int_to_roman(58), "LVIII".to_string());
    }

    #[test]
    fn for_1994() {
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_string());
    }
}
