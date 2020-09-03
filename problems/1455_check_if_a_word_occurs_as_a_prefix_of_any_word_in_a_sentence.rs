impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let search_word: Vec<_> = search_word.chars().collect();
        let mut searching = true;
        let mut index = 0;
        let mut word_count = 1;
        for c in sentence.chars() {
            if c == ' ' {
                index = 0;
                word_count += 1;
                searching = true;
                continue;
            }
            if !searching {
                continue;
            }
            if c != search_word[index] {
                searching = false;
                continue;
            }
            index += 1;            
            if index == search_word.len() {
                return word_count;
            }

        }
        return -1;
    }
}
