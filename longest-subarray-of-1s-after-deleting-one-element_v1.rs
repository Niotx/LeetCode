impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        del(nums)
    }
}
fn consecutive(nums: Vec<i32>) -> i32{
    let mut counter = 0;
    let mut max = 0;
    for i in 0..nums.len(){
        if nums[i] == 1{
            counter += 1;
            if max < counter{
                max = counter;
            }
        }else{
            counter = 0;
        }
    }
    max
}
fn del(nums: Vec<i32>) -> i32 {
    let mut max_len = 0;
    for i in 0..nums.len(){
        let mut n = nums.clone();
        n.remove(i);
        max_len = max_len.max(consecutive(n));
    }
    max_len
}
