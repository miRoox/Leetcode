pub struct Solution { }


impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i1=0;
        while i1<nums.len() {
            let mut i2=i1+1;
            while i2<nums.len() {
                if nums[i1]+nums[i2]==target {
                    return vec![i1 as i32,i2 as i32]
                }
                i2+=1
            }
            i1+=1
        }
        unreachable!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), vec![0,1]);
    }

    #[test]
    fn last_two() {
        assert_eq!(Solution::two_sum(vec![3,2,4], 6), vec![1,2]);
    }
}
