
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut answer = vec![];
        let mut mul = 0;
        let mut counter = 0;
        for j in 0..nums.len(){
                
                if j != counter {
                    mul += nums[j] * nums[counter];
                    print!("{} --",mul);
                    counter += 1;
                }   
            answer.push(mul);
            mul = 0;
            counter = 0;
        }
        return answer;
    }
}
