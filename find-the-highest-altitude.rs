impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut num = 0;
        let mut max = 0;
        for i in gain{
            num += i;
            if num > max {
                max = num;
            }
        }
        max
    }   
}
