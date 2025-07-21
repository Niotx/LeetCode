impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let candiesc = candies.clone();
        let candies_len = candies.len();
        let mut result = vec![false; candies_len];
        let max_num = candiesc.iter().max();

        for i in 0..candies_len{
            let mut canex = addex(candiesc.clone(), extra_candies, i );
            if canex >=*max_num.unwrap(){result[i] = true;}
            else{result[i] = false;}
        }
        

        return result.to_vec();
    }
}
fn addex(candies: Vec<i32>, extra_candies: i32, index: usize) -> i32{
    
    return candies[index] + extra_candies;
}
