impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {   
        let mut c = 0;
        for i in 0..grid.len(){
            let mut v:Vec<i32> = vec![];
            for j in 0..grid.len(){
                v.push(grid[j][i])
            }
            for k in 0..grid.len(){
                if v == grid[k]{
                    c += 1
                }
            }  
        }
        c
    }
}
