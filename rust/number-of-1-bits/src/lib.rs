pub struct Solution {}

impl Solution {
    #[allow(non_snake_case)]
    pub fn hammingWeight(n: u32) -> i32 {
        #[cfg(target_arch = "x86")]
        use core::arch::x86::_popcnt32;
        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64::_popcnt32;
        // Cheat!
        unsafe { _popcnt32(n as i32) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baisc() {
        assert_eq!(
            Solution::hammingWeight(0b_00000000000000000000000000001011),
            3
        );
    }

    #[test]
    fn one() {
        assert_eq!(
            Solution::hammingWeight(0b_00000000000000000000000010000000),
            1
        );
    }

    #[test]
    fn sign_bit() {
        assert_eq!(
            Solution::hammingWeight(0b_11111111111111111111111111111101),
            31
        );
    }
}
