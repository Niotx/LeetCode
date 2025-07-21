impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut schar : Vec<char> = s.chars().collect();
        let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let mut vw = vec![];
        for i in 0..schar.len(){

            for j in 0..vowels.len(){
                if schar[i] == vowels[j]{
                    vw.push(schar[i]);
                }
            }
        }
        vw.reverse();
        let mut counter = 0;
        for i in 0..schar.len(){

            if vowel_is_true(schar.clone(), i){
                schar[i] = vw[counter];
                counter += 1;
            }
        }
        return schar.into_iter().collect();
    }
}
fn vowel_is_true(s: Vec<char>, i: usize) -> bool{
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    for j in 0..vowels.len(){
        if s[i] == vowels[j]{
            return true;
        }
    }
    return false;
}
