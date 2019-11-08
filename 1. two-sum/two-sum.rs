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
