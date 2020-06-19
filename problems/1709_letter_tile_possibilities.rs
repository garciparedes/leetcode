use std::collections::HashSet;

impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut cases: HashSet<String> = HashSet::new();
        Solution::rec(&mut cases, tiles, String::new());
        return cases.len() as i32;
    }
    
    fn rec(cases: &mut HashSet<String>, tiles: String, current: String) {
        if !current.is_empty() {
            cases.insert(current.clone());
        }
        for i in 0..tiles.len() {
            Solution::rec(
                cases, 
                tiles[..i].to_string() + &tiles[i + 1..], 
                current.clone() + &tiles[i..i + 1]
            );
        }
    }
}
