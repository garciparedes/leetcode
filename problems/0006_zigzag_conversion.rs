use std::cmp;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 || s.len() == 1 {
            return s;
        }

        let mut rows: Vec<String> = vec![String::new(); cmp::min(num_rows as usize, s.len())];
        let mut row_index = 0;
        let mut direction = false;
        for c in s.chars() {
            
            rows[row_index].push(c);

            if row_index == 0 || row_index == rows.len() - 1 {
                direction = !direction;
            }
            row_index = (row_index as i32 + (direction as i32) * 2 - 1) as usize;
        }
        
        return rows.join("");
    }
}
