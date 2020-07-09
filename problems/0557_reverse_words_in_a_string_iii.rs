impl Solution {
    pub fn reverse_words(s: String) -> String {
        return s 
            .trim()
            .split(" ")
            .filter(|&x| x.trim().len() > 0)
            .map(|x| x.chars().rev().collect())
            .collect::<Vec<String>>()
            .join(" ");
    }
}
