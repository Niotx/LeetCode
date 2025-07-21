impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str1.clone() + &str2 != str2.clone() + &str1 {
            return String::new();
        }
        let s1 = str1.len();
        let s2 = str2.len();
        let f = gcd(s1,s2);
        
        return str1[0..f].to_string();
    }

    
}
fn gcd(a : usize, b : usize) -> usize{
        if b == 0 {a} else{ gcd(b,a % b)}
    }
