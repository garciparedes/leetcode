impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut result = vec![' '; s.len()];
        for (i, c) in indices.into_iter().zip(s.chars()) {
            result[i as usize] = c;
        }
        return result.into_iter().collect();
    }
}
