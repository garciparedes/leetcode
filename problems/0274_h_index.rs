impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort_unstable_by_key(|x| - x);
        
        let mut i = 0;
        while (i as usize) < citations.len() && citations[i as usize] > i {
            i += 1;
        }
        
        return i;
        
    }
}
