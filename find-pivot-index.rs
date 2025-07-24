impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = 0;
        for i in 0..nums.len(){
            for j in 0..nums.len(){
                if j < i {
                    left += nums[j];
                }
                else if j > i{
                    right += nums[j];
                }
            }
            if left == right{
                return i as i32;
            }
            left = 0;
            right = 0;
        }
        -1
    }
}
