impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let (mut left, mut right) = (0, s.len() as i32);
        
        let mut result: Vec<i32> = s
            .chars()
            .map(|c| {
                if c == 'I' {
                    let x = left;
                    left += 1;
                    return x;
                } else {
                    let x = right;
                    right -= 1;
                    return x;
                }
            })
            .collect();
        
        result.push(left);
        
        return result;
    }
}
