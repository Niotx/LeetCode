impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut sum = 0;
        let mut max = f64::MIN;
        for i in 0..((nums.len() - k as usize) + 1){
            for j in i..((k + i as i32)as usize){
                sum += nums[j as usize]; 
            }
            if (sum as f64)/(k as f64) > max{
                max = (sum as f64)/(k as f64) ;  
            }
            sum = 0;
        }
        return max.into();
    }
}
