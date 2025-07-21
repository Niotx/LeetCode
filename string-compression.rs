impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut s = String::new();
        let mut counter = 0;
        let mut c = chars[0];
        if chars.is_empty() {
            return 0;
        }
        s.push(chars[0]);
        for i in 0..(chars.len()){
            if chars[i] == c{
                counter += 1;    
                }
            if chars[i] != c {
                c = chars[i];
                if counter > 1{
                    s.push_str(&counter.to_string());                    
                }
                s.push(chars[i]);
                counter = 1;
            }
            if i == (chars.len()-1){
                if counter > 1{
                    s.push_str(&counter.to_string());  
                }
            }
        }   
        *chars = s.chars().collect();
        return chars.len() as i32; 
    }
}
