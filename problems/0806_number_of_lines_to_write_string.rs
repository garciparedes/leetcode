impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let iterable = s
            .chars()
            .map(|c| widths[(c as u8 - 97) as usize]);
        
        let mut line_count = 1;
        let mut char_count = 0;
        for width in iterable {    
            if char_count + width > 100 {
                line_count += 1;
                char_count = width;
            } else {
                char_count += width;
            }
        }
        
        return vec![line_count, char_count];
    }
}
