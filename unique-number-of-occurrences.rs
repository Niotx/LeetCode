impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut num = vec![];
        let mut count = vec![];
        let mut c = 0;
        for i in &arr{
            if !num.contains(&i){
                num.push(i);
            }
        }
        for y in 0..num.len(){
            for j in 0..arr.len(){
                if *num[y] == arr[j]{
                    c += 1;
                }
            }
            count.push(c);
            c = 0;
        }
        for m in 0..count.len(){
            for n in 0..count.len(){
                if count[m] == count[n]{
                    c += 1;
                }
            }
            if c > 1{
                return false;
            }
            c = 0;
        }
        
        true
    }
    
}
