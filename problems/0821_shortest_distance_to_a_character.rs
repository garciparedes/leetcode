impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        
        let mut positions = Vec::new();
        for i in 0..n {
            if s[i] == c {
                positions.push(i as i32);
            }
        }
        
        return (0..n as i32)
            .map(|x| {
                let mut i = 0;
                let mut best = i32::max_value();
                while i < positions.len() && i32::abs(positions[i] - x) < best {
                    best = i32::abs(positions[i] - x);
                    i += 1;
                }
                return best;
            }).collect();
    }
}
