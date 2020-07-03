use std::collections::{
    HashMap,
    HashSet,
};

impl Solution {
    pub fn prison_after_n_days(mut cells: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let m = cells.len();
        let mut options = HashSet::new();
        let mut memory = HashMap::new();
        for _ in 0..n {
            if memory.get(&cells).is_some() {
                break;
            }
            let mut cached = cells.clone();
            for i in 1..m - 1 {
                cells[i] = (cached[i - 1] == cached[i + 1]) as i32;
            }
            cells[0] = 0;
            cells[m - 1] = 0;
            memory.insert(cached, cells.clone());
            options.insert(cells.clone());
        }
        if n < memory.len() {
            return cells;
        }
        let k = (n - memory.len()) % options.len();
        for _ in  0..k  {
            cells = memory.get(&cells).unwrap().clone();
        }
        return cells;
    }
}
