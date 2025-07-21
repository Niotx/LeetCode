impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let s_chars: Vec<char> = s.chars().collect();
        let mut counter = 0;
        let mut max_v = 0;
        let k_usize = k as usize;

        for i in 0..s_chars.len() {
            if vowels.contains(&s_chars[i]) {
                counter += 1;
            }
            if i >= k_usize {
                if vowels.contains(&s_chars[i - k_usize]) {
                    counter -= 1;
                }
            }
            if counter > max_v {
                max_v = counter;
            }
        }

        max_v
    }
}
