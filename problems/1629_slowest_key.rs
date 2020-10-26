use std::cmp;

impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let mut times = vec![0; 26];
        
        for (i, c) in keys_pressed.chars().enumerate() {
            let k = (c as u8 - 'a' as u8) as usize;
            times[k] = cmp::max(times[k], release_times[i] - release_times.get(i - 1).unwrap_or(&0));
        }
        
        let mut j = 0;
        for i in 0..26 {
            if times[i] >= times[j] {
                j = i;
            }
        }
        
        return (j as u8 + 'a' as u8) as char;
    }
}

