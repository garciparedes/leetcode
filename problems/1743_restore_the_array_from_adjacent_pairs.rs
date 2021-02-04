use std::collections::HashMap;

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut contiguous = HashMap::new();
        for pair in &adjacent_pairs {
            contiguous.entry(pair[0]).or_insert_with(Vec::new).push(pair[1]);
            contiguous.entry(pair[1]).or_insert_with(Vec::new).push(pair[0]);
        }
        
        let mut ans = Vec::new();
        for (k, vs) in contiguous.iter() {
            if vs.len() == 1 {
                ans.push(*k);
                ans.push(vs[0]);
                break;
            }
        }
        
        while ans.len() < adjacent_pairs.len() + 1 {
            let vs = &contiguous[&ans[ans.len() - 1]];
            if vs[0] != ans[ans.len() - 2] {
                ans.push(vs[0]);
            } else {
                ans.push(vs[1]);
            }
        }
        
        return ans;
        
        
    }
}
