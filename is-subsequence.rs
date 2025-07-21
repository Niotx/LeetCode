impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        if s.is_empty(){
            return true;
        }
        let mut f = 0;
        let mut n = 0;
        for i in 0..t.len(){
            if t[i] == s[n]{
                if s.len()>1 {
                    n += 1;
                    f +=1;
                }else{
                    f=1;
                }  
            }
            if n == s.len(){
                break;
            }  
        }
        if f == s.len(){
            return true;
        }else{
            return false;
        }
    }
}
