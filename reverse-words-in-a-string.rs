impl Solution {
    pub fn reverse_words(s: String) -> String {
        let words: Vec<&str> = s.split_whitespace().collect();
        let reversed = words.into_iter().rev().collect::<Vec<&str>>();
        reversed.join(" ").to_string()
    }
}
