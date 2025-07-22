impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        flip(nums,k)
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
fn flip(nums: Vec<i32>, k: i32) -> i32{
    let mut max = 0;
    for i in 0..nums.len(){
        let mut counter = k;
        let mut v = nums.clone();
        for j in i..nums.len(){
            if v[j] == 0 && counter > 0{
                v[j] = 1;
                counter -= 1;
            }
        }
        let mut cons = consecutive(v.clone());
        if max < cons{
            max = cons;
        }
    }
    max
}
