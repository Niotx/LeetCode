impl Solution {
    pub fn max_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut op = 0;
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right{
            let mut sum = nums[left] + nums[right];
            if sum == k{
                op += 1;
                left += 1;
                right -= 1;
            }else if sum < k{
                left += 1;
            }else{
                right -= 1;
            }
        }
        op
        }
}
